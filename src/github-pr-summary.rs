use flowsnet_platform_sdk::write_error_log;
use github_flows::{
    listen_to_event, octocrab::models::events::payload::PullRequestEventAction, EventPayload,
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
    let login = "jaykchen";
    let owner = "jaykchen";
    let repo = "vitesse-lite";
    let openai_key_name = "jaykchen";

    listen_to_event(login, owner, repo, vec!["pull_request"], |payload| {
        handler(owner, repo, openai_key_name, payload)
    })
    .await;

    Ok(())
}

async fn handler(owner: &str, repo: &str, openai_key_name: &str, payload: EventPayload) {
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

    let diff_url =
        "https://patch-diff.githubusercontent.com/raw/WasmEdge/WasmEdge/pull/2368.diff".to_string();
    // let diff_url = format!(
    //     "https://patch-diff.githubusercontent.com/raw/{owner}/{repo}/pull/{pull_number}.diff"
    // );

    let prompt = format!("@chatgptbot, read {diff_url}, which is the code diff document for a pull request on GitHub, please summarize it into key points");
    // let prompt_start = format!("Contributor {contributor} filed the pull request titled {title}, proposing changes as shown in plain text diff record at the end of this message");

    // let chunked = chunk_input(diff_as_text.to_string());
    // let mut prompt_arr = serial_prompts(chunked, prompt_start);
    // let count = prompt_arr.len();
    // let mut count_down = count.clone();
    // while let Some(prompt) = prompt_arr.pop() {
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
        // if count_down == count {
        //     send_message_to_channel(
        //         "ik8",
        //         "general",
        //         "please wait..., it could take minutes to summarize a large pull request"
        //             .to_string(),
        //     );
        // } else if count_down == 1 {
        send_message_to_channel("ik8", "general", r.choice);
        // }
        // count_down -= 1;
    }
}
