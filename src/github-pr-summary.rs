use dotenv::dotenv;
use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    get_octo, listen_to_event,
    octocrab::models::events::payload::{IssueCommentEventAction, PullRequestEventAction},
    EventPayload,
};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use openai_flows::{chat_completion, ChatModel, ChatOptions};
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    dotenv().ok();

    let login = env::var("login").unwrap_or("juntao".to_string());
    let owner = env::var("owner").unwrap_or("juntao".to_string());
    let repo = env::var("repo").unwrap_or("test".to_string());
    let openai_key_name = env::var("openai_key_name").unwrap_or("chatmichael".to_string());
    let trigger_phrase = env::var("trigger_phrase").unwrap_or("flows summarize".to_string());

    let events = vec!["pull_request", "issue_comment"];
    listen_to_event(&login, &owner, &repo, events, |payload| {
        handler(&login, &owner, &repo, &openai_key_name, &trigger_phrase, payload)
    })
    .await;

    Ok(())
}

async fn handler(
    login: &str,
    owner: &str,
    repo: &str,
    openai_key_name: &str,
    trigger_phrase: &str,
    payload: EventPayload,
) {
    let (_title, pull_number, _contributor) = match payload {
        EventPayload::PullRequestEvent(e) => {
            if e.action == PullRequestEventAction::Closed {
                write_error_log!("Closed pull event");
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
                write_error_log!("Deleted issue event");
                return;
            }

            let body = e.comment.body.unwrap_or_default();

            // if e.comment.performed_via_github_app.is_some() {
            //     return;
            // }
            // TODO: Makeshift but operational
            if body.starts_with("Hello, I am a [serverless review bot]") {
                write_error_log!("Ignore comment via bot");
                return;
            };

            if !body.to_lowercase().contains(&trigger_phrase.to_lowercase()) {
                write_error_log!(format!("Ignore the comment, raw: {}", body));
                return;
            }

            (e.issue.title, e.issue.number, e.issue.user.login)
        }
        _ => return,
    };

    let octo = get_octo(Some(String::from(login)));
    let issues = octo.issues(owner, repo);

    let chat_id = format!("PR#{pull_number}");

    // let patch_url =
    //     "https://patch-diff.githubusercontent.com/raw/WasmEdge/WasmEdge/pull/2368.patch"
    //         .to_string();
    let patch_url = format!(
        "https://patch-diff.githubusercontent.com/raw/{owner}/{repo}/pull/{pull_number}.patch"
    );
    let patch_uri = Uri::try_from(patch_url.as_str()).unwrap();
    let mut writer = Vec::new();
    let _ = Request::new(&patch_uri)
        .method(Method::GET)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "Flows Network Connector")
        .send(&mut writer)
        .map_err(|_e| {})
        .unwrap();
    let patch_as_text = String::from_utf8_lossy(&writer);

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
        // Append the line to the current commit if the current commit is less than 8000 chars (the
        // max token size is 4096)
        if current_commit.len() < 8000 {
            current_commit.push_str(line);
            current_commit.push('\n');
        }
    }
    if !current_commit.is_empty() {
        // Store the last commit
        commits.push(current_commit.clone());
    }
    // write_error_log!(&format!("Num of commits = {}", commits.len()));

    if commits.is_empty() {
        write_error_log!("Cannot parse any commit from the patch file");
        return;
    }

    let mut reviews: Vec<String> = Vec::new();
    let mut reviews_text = String::new();
    for (_i, commit) in commits.iter().enumerate() {
        let prompt = "You will act as a reviewer for GitHub Pull Requests. The next message is a GitHub patch for a single commit. Please summarize the key changes and identify potential problems.";
        let co = ChatOptions {
            model: ChatModel::GPT35Turbo,
            restart: true,
            restarted_sentence: Some(prompt),
        };
        if let Some(r) = chat_completion(openai_key_name, &chat_id, commit, &co) {
            write_error_log!("Got a patch summary");
            reviews_text.push_str("------\n");
            reviews_text.push_str(&r.choice);
            reviews_text.push('\n');
            reviews.push(r.choice);
        }
    }

    let mut resp = String::new();
    resp.push_str("Hello, I am a [serverless review bot](https://github.com/flows-network/github-pr-summary/) on [flows.network](https://flows.network/). Here are my reviews of code commits in this PR.\n\n------\n\n");
    if reviews.len() > 1 {
        let prompt = "In the next messge, I will provide a set of reviews for code patches. Each review starts with a ------ line. Please write a summary of all the reviews";
        let co = ChatOptions {
            model: ChatModel::GPT35Turbo,
            restart: true,
            restarted_sentence: Some(prompt),
        };
        if let Some(r) = chat_completion(openai_key_name, &chat_id, &reviews_text, &co) {
            write_error_log!("Got the overall summary");
            resp.push_str(&r.choice);
            resp.push_str("\n\n## Details\n\n");
        }
    }
    for (i, review) in reviews.iter().enumerate() {
        resp.push_str(&format!("### Commit {}\n", i + 1));
        resp.push_str(review);
        resp.push_str("\n\n");
    }
    // Send the entire response to GitHub PR
    issues.create_comment(pull_number, resp).await.unwrap();
}
