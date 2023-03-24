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
use words_count::count;
#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let owner = "jaykchen";
    let repo = "vitesse-lite";
    let openai_key_name = "jaykchen";

    listen_to_event(owner, repo, vec!["pull_request"], |payload| {
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

    // diff_url: https://patch-diff.githubusercontent.com/raw/jaykchen/vitesse-lite/pull/45.diff
    let diff_url = "https://patch-diff.githubusercontent.com/raw/WasmEdge/WasmEdge/pull/2369.diff";
    // let diff_url = format!(
    //     "https://patch-diff.githubusercontent.com/raw/{owner}/{repo}/pull/{pull_number}.diff"
    // );

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

    let prompt_start = format!("Contributor {contributor} filed the pull request titled {title}, proposing changes as shown in plain text diff record at the end of this message");

    match maybe_split_long_input(&diff_as_text) {
        None => {
            let prompt = format!("{prompt_start}, please summarize into key points by order of importance: {diff_as_text}");

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
        Some(multiple_segments) => {
            let mut text_vec = serial_prompts(multiple_segments, prompt_start);
            while let Some(prompt) = text_vec.pop() {
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
        }
    };
}

pub fn maybe_split_long_input(inp: &str) -> Option<Vec<String>> {
    let words_count = count(inp);

    if words_count.words > 2000 && words_count.characters > 8000 {
        let mut diff_pcs = String::new();

        let mut diff_str_vec = Vec::new();

        for chunk in inp.split_inclusive("diff --git") {
            let temp = diff_pcs.clone();

            diff_pcs.push_str(chunk);

            if diff_pcs.len() > 8000 {
                diff_str_vec.push(temp);
                diff_pcs.clear();
                diff_pcs.push_str(chunk);
            }
        }
        return Some(diff_str_vec);
    }
    None
}

pub fn serial_prompts(inp_vec: Vec<String>, prompt_start: String) -> Vec<String> {
    let mut prompt_vec = Vec::new();
    let mut inp_vec = inp_vec.clone();
    let diff_as_text = inp_vec.pop().unwrap();
    let first_prompt = format!("{prompt_start}, please summarize into key points by order of importance, this long file is to be sent in batches, please repond by saying [receiving incoming data] before the last input, here is number 1 : {diff_as_text}");
    prompt_vec.push(first_prompt.clone());

    match inp_vec.len() {
        1 => {
            let diff_as_text = inp_vec[0].clone();
            let end_prompt = format!(
                "Please give me the summary, here is last part of the input : {diff_as_text}"
            );
            prompt_vec.push(end_prompt.clone());
            prompt_vec
        }
        2.. => {
            let mut serial_number = 2;
            let mut last = String::new();
            while let Some(diff_as_text) = inp_vec.last() {
                let mid_prompt = format!("please repond by saying [receiving incoming data] before the last input, here is number {serial_number} : {diff_as_text}");

                prompt_vec.push(mid_prompt);
                serial_number += 1;
                last = inp_vec.pop().unwrap();
            }
            let diff_as_text = last;
            let end_prompt = format!(
                "Please give me the summary, here is last part of the input : {diff_as_text}"
            );
            prompt_vec.push(end_prompt);
            prompt_vec
        }
        _ => {
            vec![]
        }
    }
}
