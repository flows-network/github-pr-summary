use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use github_flows::{
    event_handler, get_octo, listen_to_event,
    octocrab::models::CommentId,
    octocrab::models::webhook_events::{WebhookEvent, WebhookEventPayload},
    octocrab::models::webhook_events::payload::{IssueCommentWebhookEventAction, PullRequestWebhookEventAction},
    GithubLogin,
};
use llmservice_flows::{
    chat::{ChatOptions},
    LLMServiceFlows,
};
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    dotenv().ok();
    logger::init();
    log::debug!("Running github-pr-summary/main");

    let owner = env::var("github_owner").unwrap_or("juntao".to_string());
    let repo = env::var("github_repo").unwrap_or("test".to_string());

    listen_to_event(&GithubLogin::Default, &owner, &repo, vec!["pull_request", "issue_comment"]).await;
}

#[event_handler]
async fn handler(event: Result<WebhookEvent, serde_json::Error>) {
    dotenv().ok();
    logger::init();
    log::debug!("Running github-pr-summary/main handler()");

    let owner = env::var("github_owner").unwrap_or("juntao".to_string());
    let repo = env::var("github_repo").unwrap_or("test".to_string());
    let trigger_phrase = env::var("trigger_phrase").unwrap_or("flows summarize".to_string());
    let llm_api_endpoint = env::var("llm_api_endpoint").unwrap_or("https://api.openai.com/v1".to_string());
    let llm_model_name = env::var("llm_model_name").unwrap_or("gpt-4o".to_string());
    let llm_ctx_size = env::var("llm_ctx_size").unwrap_or("16384".to_string()).parse::<u32>().unwrap_or(0);
    let llm_api_key = env::var("llm_api_key").unwrap_or("LLAMAEDGE".to_string());

    //  The soft character limit of the input context size
    //  This is measured in chars. We set it to be 2x llm_ctx_size, which is measured in tokens.
    let ctx_size_char : usize = (2 * llm_ctx_size).try_into().unwrap_or(0);

    let payload = event.unwrap();
    let mut new_commit : bool = false;
    let (title, pull_number, _contributor) = match payload.specific {
        WebhookEventPayload::PullRequest(e) => {
            if e.action == PullRequestWebhookEventAction::Opened {
                log::debug!("Received payload: PR Opened");
            } else if e.action == PullRequestWebhookEventAction::Synchronize {
                new_commit = true;
                log::debug!("Received payload: PR Synced");
            } else {
                log::debug!("Not an Opened or Synchronize event for PR");
                return;
            }
            let p = e.pull_request;
            (
                p.title.unwrap_or("".to_string()),
                p.number,
                p.user.unwrap().login,
            )
        }
        WebhookEventPayload::IssueComment(e) => {
            if e.action == IssueCommentWebhookEventAction::Deleted {
                log::debug!("Deleted issue comment");
                return;
            }
            log::debug!("Other event for issue comment");

            let body = e.comment.body.unwrap_or_default();

            // if e.comment.performed_via_github_app.is_some() {
            //     return;
            // }
            // TODO: Makeshift but operational
            if body.starts_with("Hello, I am a [PR summary agent]") {
                log::info!("Ignore comment via bot");
                return;
            };

            if !body.to_lowercase().starts_with(&trigger_phrase.to_lowercase()) {
                log::info!("Ignore the comment without the magic words");
                return;
            }

            (e.issue.title, e.issue.number, e.issue.user.login)
        }
        _ => return,
    };

    let octo = get_octo(&GithubLogin::Default);
    let issues = octo.issues(owner.clone(), repo.clone());
    let mut comment_id: CommentId = 0u64.into();
    if new_commit {
        // Find the first "Hello, I am a [PR summary agent]" comment to update
        match issues.list_comments(pull_number).send().await {
            Ok(comments) => {
                for c in comments.items {
                    if c.body.unwrap_or_default().starts_with("Hello, I am a [PR summary agent]") {
                        comment_id = c.id;
                        break;
                    }
                }
            }
            Err(error) => {
                log::error!("Error getting comments: {}", error);
                return;
            }
        }
    } else {
        // PR OPEN or Trigger phrase: create a new comment
        match issues.create_comment(pull_number, "Hello, I am a [PR summary agent](https://github.com/flows-network/github-pr-summary/) on [flows.network](https://flows.network/).\n\nIt could take a few minutes for me to analyze this PR. Relax, grab a cup of coffee and check back later. Thanks!").await {
            Ok(comment) => {
                comment_id = comment.id;
            }
            Err(error) => {
                log::error!("Error posting comment: {}", error);
                return;
            }
        }
    }
    if comment_id == 0u64.into() { return; }

    let pulls = octo.pulls(owner.clone(), repo.clone());
    let patch_as_text = pulls.get_patch(pull_number).await.unwrap();
    let mut current_commit = String::new();
    let mut commits: Vec<String> = Vec::new();
    for line in patch_as_text.lines() {
        if line.starts_with("From ") {
            // Detected a new commit
            if !current_commit.is_empty() {
                // Store the previous commit
                commits.push(current_commit.clone());
            }
            // Start a new commit
            current_commit.clear();
        }
        // Append the line to the current commit if the current commit is less than ctx_size_char
        if current_commit.len() < ctx_size_char {
            current_commit.push_str(line);
            current_commit.push('\n');
        }
    }
    if !current_commit.is_empty() {
        // Store the last commit
        commits.push(current_commit.clone());
    }

    if commits.is_empty() {
        log::error!("Cannot parse any commit from the patch file");
        return;
    }

    let chat_id = format!("PR#{pull_number}");
    let system = &format!("You are an experienced software developer. You will act as a reviewer for a GitHub Pull Request titled \"{}\". Please be as concise as possible while being accurate.", title);
    let mut lf = LLMServiceFlows::new(&llm_api_endpoint);
    lf.set_api_key(&llm_api_key);
    // lf.set_retry_times(3);

    let mut reviews: Vec<String> = Vec::new();
    let mut reviews_text = String::new();
    for (_i, commit) in commits.iter().enumerate() {
        let commit_hash = &commit[5..45];
        log::debug!("Sending patch to LLM: {}", commit_hash);
        let co = ChatOptions {
            model: Some(&llm_model_name),
            token_limit: llm_ctx_size,
            restart: true,
            system_prompt: Some(system),
            ..Default::default()
        };
        let question = "The following is a GitHub patch. Please summarize the key changes in concise points. Start with the most important findings.\n\n".to_string() + truncate(commit, ctx_size_char);
        match lf.chat_completion(&chat_id, &question, &co).await {
            Ok(r) => {
                if reviews_text.len() < ctx_size_char {
                    reviews_text.push_str("------\n");
                    reviews_text.push_str(&r.choice);
                    reviews_text.push_str("\n");
                }
                let mut review = String::new();
                review.push_str(&format!("### [Commit {commit_hash}](https://github.com/{owner}/{repo}/pull/{pull_number}/commits/{commit_hash})\n"));
                review.push_str(&r.choice);
                review.push_str("\n\n");
                reviews.push(review);
                log::debug!("Received LLM resp for patch: {}", commit_hash);
            }
            Err(e) => {
                log::error!("LLM returned an error for commit {commit_hash}: {}", e);
            }
        }
    }

    let mut resp = String::new();
    resp.push_str("Hello, I am a [PR summary agent](https://github.com/flows-network/github-pr-summary/) on [flows.network](https://flows.network/). Here are my reviews of code commits in this PR.\n\n------\n\n");
    if reviews.len() > 1 {
        log::debug!("Sending all reviews to LLM for summarization");
        let co = ChatOptions {
            model: Some(&llm_model_name),
            token_limit: llm_ctx_size,
            restart: true,
            system_prompt: Some(system),
            ..Default::default()
        };
        let question = "Here is a set of summaries for source code patches in this PR. Each summary starts with a ------ line. Write an overall summary. Present the potential issues and errors first, following by the most important findings, in your summary.\n\n".to_string() + &reviews_text;
        match lf.chat_completion(&chat_id, &question, &co).await {
            Ok(r) => {
                resp.push_str(&r.choice);
                resp.push_str("\n\n## Details\n\n");
                log::debug!("Received the overall summary");
            }
            Err(e) => {
                log::error!("LLM returned an error for the overall summary: {}", e);
            }
        }
    }
    for (_i, review) in reviews.iter().enumerate() {
        resp.push_str(review);
    }

    // Send the entire response to GitHub PR
    // issues.create_comment(pull_number, resp).await.unwrap();
    match issues.update_comment(comment_id, resp).await {
        Err(error) => {
            log::error!("Error posting resp: {}", error);
        }
        _ => {}
    }
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
