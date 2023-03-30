use flowsnet_platform_sdk::write_error_log;
use github_flows::{
     get_octo, listen_to_event, octocrab::models::events::payload::PullRequestEventAction, EventPayload,
};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use openai_flows::{chat_completion, ChatModel, ChatOptions};
use dotenv::dotenv;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    dotenv().ok();

    let login: String = match env::var("login") {
        Err(_) => "juntao".to_string(),
        Ok(name) => name,
    };

    let owner: String = match env::var("owner") {
        Err(_) => "juntao".to_string(),
        Ok(name) => name,
    };

    let repo: String = match env::var("repo") {
        Err(_) => "test".to_string(),
        Ok(name) => name,
    };

    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    listen_to_event(&login, &owner, &repo, vec!["pull_request"], |payload| {
        handler(&login, &owner, &repo, &openai_key_name, payload)
    })
    .await;

    Ok(())
}

async fn handler(login: &str, owner: &str, repo: &str, openai_key_name: &str, payload: EventPayload) {
    let octo = get_octo(Some(String::from(login)));
    let issues = octo.issues(owner, repo);
    let mut pull = None;

    match payload {
        EventPayload::PullRequestEvent(e) => {
            if e.action == PullRequestEventAction::Closed {
                return;
            }
            pull = Some(e.pull_request);
        }

        _ => (),
    };

    let (_title, pull_number, _contributor) = match pull {
        Some(p) => (
            p.title.unwrap_or("".to_string()),
            p.number,
            p.user.unwrap().login,
        ),
        None => return,
    };
    let chat_id = &format!("PR#{}", pull_number);

    let patch_url =
        "https://patch-diff.githubusercontent.com/raw/WasmEdge/WasmEdge/pull/2368.patch".to_string();
    // let patch_url = format!(
    //     "https://patch-diff.githubusercontent.com/raw/{owner}/{repo}/pull/{pull_number}.patch"
    // );
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
    
    let mut commits: Vec<String> = Vec::new();
    let mut reviews: Vec<String> = Vec::new();
    let mut current_commit = String::new();
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
        // Append the line to the current commit
        current_commit.push_str(&line);
        current_commit.push('\n');
    }

    if commits.len() < 1 {
        write_error_log!("Cannot parse any commit from the patch file");
        return;
    }

    // Ask the initial prompt
    let prompt = "In this thread, you will act as a reviewer for GitHub Pull Requests. I will send you several GitHub patch files, each containing a patch for a single commit. Please summarize the key changes in each patch and identify potential problems. Once I sent all the patches, I will say 'That is it' and you will provide a summary of all patches in this conversation.";
    // ChatOption for the initial prompt
    let co = ChatOptions {
        model: ChatModel::GPT35Turbo,
        restart: true,
        restarted_sentence: Some(prompt),
    };
    // Start the session with the prompt, and send in the first commit
    write_error_log!(format!("Commit 1:\n{}", &commits[0]));
    chat_completion(openai_key_name, chat_id, &commits[0], &co);


    // ChatOption for all subsequent questions
    let co = ChatOptions {
        model: ChatModel::GPT35Turbo,
        restart: false,
        restarted_sentence: None,
    };
    for (i, commit) in commits.iter().enumerate() {
        // skip the first commit as it has been asked
        if i == 0 { continue; }
        write_error_log!(format!("Commit {}:\n{}", i + 1, commit));
        if let Some(r) = chat_completion(openai_key_name, chat_id, &commit, &co) {
            write_error_log!(r.choice);
            reviews.push(r.choice);
        }
    }

    // Conclude the session
    if let Some(r) = chat_completion(openai_key_name, chat_id, "That is it", &co) {
        let mut resp = String::new();
        resp.push_str(&r.choice);
        resp.push_str("\n\n# Details\n\n");
        for (_i, review) in reviews.iter().enumerate() {
            resp.push_str(&review);
            resp.push_str("\n\n");
        }
        // Send the entire response to GitHub PR
        issues.create_comment(pull_number, resp).await.unwrap();
    }
}
