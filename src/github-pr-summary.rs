use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use github_flows::{
    event_handler, get_octo, listen_to_event,
    // octocrab::models::{events::payload::EventPayload, reactions::ReactionContent},
    octocrab::models::events::payload::{EventPayload, IssueCommentEventAction, PullRequestEventAction},
    octocrab::models::CommentId,
    GithubLogin,
};
use llmservice_flows::{
    chat::{ChatOptions},
    LLMServiceFlows,
};
use std::env;

//  The soft character limit of the input context size
//  THe codestral has a context length of 32k, and we allow 8k context here
static CHAR_SOFT_LIMIT : usize = 8192;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    dotenv().ok();
    logger::init();
    log::debug!("Running github-pr-summary/main");

    let owner = env::var("github_owner").unwrap_or("juntao".to_string());
    let repo = env::var("github_repo").unwrap_or("test".to_string());

    listen_to_event(&GithubLogin::Provided(owner.clone()), &owner, &repo, vec!["pull_request", "issue_comment"]).await;
}

#[event_handler]
async fn handler(payload: EventPayload) {
    dotenv().ok();
    logger::init();
    log::debug!("Running github-pr-summary/main handler()");

    let owner = env::var("github_owner").unwrap_or("juntao".to_string());
    let repo = env::var("github_repo").unwrap_or("test".to_string());
    let trigger_phrase = env::var("trigger_phrase").unwrap_or("flows summarize".to_string());
    let llm_api_endpoint = env::var("llm_api_endpoint").unwrap_or("https://codestral-01-22b.us.gaianet.network/v1".to_string());
    let llm_model_name = env::var("llm_model_name").unwrap_or("Codestral-22B-v0.1-hf-Q5_K_M".to_string());

    /*
    if let EventPayload::IssueCommentEvent(e) = payload {
        let comment_id = e.comment.id.0;

        // installed app login
        let octo = get_octo(&GithubLogin::Provided(String::from("some_login")));

        let _reaction = octo
            .issues("some_owner", "some_repo")
            .create_comment_reaction(comment_id, ReactionContent::Rocket)
            .await
            .unwrap();
    };
    */

    let mut new_commit : bool = false;
    let (title, pull_number, _contributor) = match payload {
        EventPayload::PullRequestEvent(e) => {
            if e.action == PullRequestEventAction::Opened {
                log::debug!("Received payload: PR Opened");
            } else if e.action == PullRequestEventAction::Synchronize {
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
        EventPayload::IssueCommentEvent(e) => {
            if e.action == IssueCommentEventAction::Deleted {
                log::debug!("Deleted issue comment");
                return;
            }
            log::debug!("Other event for issue comment");

            let body = e.comment.body.unwrap_or_default();

            // if e.comment.performed_via_github_app.is_some() {
            //     return;
            // }
            // TODO: Makeshift but operational
            if body.starts_with("Hello, I am a [code review bot]") {
                log::info!("Ignore comment via bot");
                return;
            };

            if !body.to_lowercase().contains(&trigger_phrase.to_lowercase()) {
                log::info!("Ignore the comment without the magic words");
                return;
            }

            (e.issue.title, e.issue.number, e.issue.user.login)
        }
        _ => return,
    };

    let octo = get_octo(&GithubLogin::Provided(owner.clone()));
    let issues = octo.issues(owner.clone(), repo.clone());
    let mut comment_id: CommentId = 0u64.into();
    if new_commit {
        // Find the first "Hello, I am a [code review bot]" comment to update
        match issues.list_comments(pull_number).send().await {
            Ok(comments) => {
                for c in comments.items {
                    if c.body.unwrap_or_default().starts_with("Hello, I am a [code review bot]") {
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
        match issues.create_comment(pull_number, "Hello, I am a [code review bot](https://github.com/flows-network/github-pr-summary/) on [flows.network](https://flows.network/).\n\nIt could take a few minutes for me to analyze this PR. Relax, grab a cup of coffee and check back later. Thanks!").await {
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
        // Append the line to the current commit if the current commit is less than CHAR_SOFT_LIMIT
        if current_commit.len() < CHAR_SOFT_LIMIT {
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
    let system = &format!("You are an experienced software developer. You will act as a reviewer for a GitHub Pull Request titled \"{}\".", title);
    let mut lf = LLMServiceFlows::new(&llm_api_endpoint);
    lf.set_retry_times(3);

    let mut reviews: Vec<String> = Vec::new();
    let mut reviews_text = String::new();
    for (_i, commit) in commits.iter().enumerate() {
        let commit_hash = &commit[5..45];
        log::debug!("Sending patch to OpenAI: {}", commit_hash);
        let co = ChatOptions {
            model: Some(&llm_model_name),
            restart: true,
            system_prompt: Some(system),
            ..Default::default()
        };
        let question = "The following is a GitHub patch. Please summarize the key changes and identify potential problems. Start with the most important findings.\n\n".to_string() + truncate(commit, CHAR_SOFT_LIMIT);
        match lf.chat_completion(&chat_id, &question, &co).await {
            Ok(r) => {
                if reviews_text.len() < CHAR_SOFT_LIMIT {
                    reviews_text.push_str("------\n");
                    reviews_text.push_str(&r.choice);
                    reviews_text.push_str("\n");
                }
                let mut review = String::new();
                review.push_str(&format!("### [Commit {commit_hash}](https://github.com/{owner}/{repo}/pull/{pull_number}/commits/{commit_hash})\n"));
                review.push_str(&r.choice);
                review.push_str("\n\n");
                reviews.push(review);
                log::debug!("Received OpenAI resp for patch: {}", commit_hash);
            }
            Err(e) => {
                log::error!("OpenAI returned an error for commit {commit_hash}: {}", e);
            }
        }
    }

    let mut resp = String::new();
    resp.push_str("Hello, I am a [code review bot](https://github.com/flows-network/github-pr-summary/) on [flows.network](https://flows.network/). Here are my reviews of code commits in this PR.\n\n------\n\n");
    if reviews.len() > 1 {
        log::debug!("Sending all reviews to OpenAI for summarization");
        let co = ChatOptions {
            model: Some(&llm_model_name),
            restart: true,
            system_prompt: Some(system),
            ..Default::default()
        };
        let question = "Here is a set of summaries for software source code patches. Each summary starts with a ------ line. Please write an overall summary considering all the individual summary. Please present the potential issues and errors first, following by the most important findings, in your summary.\n\n".to_string() + &reviews_text;
        match lf.chat_completion(&chat_id, &question, &co).await {
            Ok(r) => {
                resp.push_str(&r.choice);
                resp.push_str("\n\n## Details\n\n");
                log::debug!("Received the overall summary");
            }
            Err(e) => {
                log::error!("OpenAI returned an error for the overall summary: {}", e);
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
