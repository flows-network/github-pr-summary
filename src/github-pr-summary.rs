use github_flows::{
    get_octo, listen_to_event, octocrab::models::events::payload::PullRequestEventAction,
    EventPayload,
};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use openai_flows::{chat_completion, ChatModel, ChatOptions};
use slack_flows::send_message_to_channel;
use tokio::*;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let owner = "jaykchen";
    let repo = "vitesse-lite";

    listen_to_event(owner, repo, vec!["pull_request"], |payload| {
        handler(owner, repo, payload)
    })
    .await;

    Ok(())
}

async fn handler(owner: &str, repo: &str, payload: EventPayload) {
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

    let (title, pull_number, contributor) = match pull {
        Some(p) => (
            p.title.unwrap_or("".to_string()),
            p.number,
            p.user.unwrap().login,
        ),
        None => return,
    };

    // commits_url:  "https://api.github.com/repos/jaykchen/vitesse-lite/pulls/46/commits",
    // diff_url: https://patch-diff.githubusercontent.com/raw/jaykchen/vitesse-lite/pull/45.diff
    let openai_key_name = "jaykchen";
    let diff_url = format!(
        "https://patch-diff.githubusercontent.com/raw/{owner}/{repo}/pull/{pull_number}.diff"
    );
    send_message_to_channel("ik8", "general", diff_url.to_string());

    let uri = Uri::try_from(diff_url.as_str()).unwrap();

    let mut writer = Vec::new();

    let _ = Request::new(&uri)
        .method(Method::GET)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "Github Connector of Second State Reactor")
        .send(&mut writer)
        .map_err(|_e| {})
        .unwrap();

    let diff_as_text = String::from_utf8_lossy(&writer);

    let prompt = format!("Contributor {contributor} filed the pull request titled {title}, proposing changes as shown in plain text diff record at the end of this message, please summarize into key points by order of importance: {diff_as_text}");
    send_message_to_channel("ik8", "general", prompt.to_string());

    let co = ChatOptions {
        model: ChatModel::GPT35Turbo,
        restart: true,
        restarted_sentence: Some(&prompt),
    };

    if let Some(r) = chat_completion(
        openai_key_name,
        &format!("PR#{}", pull_number),
        &prompt,
        &co,
    ) {
        send_message_to_channel("ik8", "general", r.choice);
    }
}
