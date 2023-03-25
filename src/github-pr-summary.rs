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
        "https://patch-diff.githubusercontent.com/raw/WasmEdge/WasmEdge/pull/2366.diff".to_string();
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

    if diff_as_text.lines().count() < 210 {
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
    } else {
        let chunked = chunk_input(diff_as_text.to_string());
        let mut prompt_arr = serial_prompts(chunked, prompt_start);
        let count = prompt_arr.len();
        let mut count_down = count.clone();
        while let Some(prompt) = prompt_arr.pop() {
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
                if count_down == count {
                    send_message_to_channel(
                        "ik8",
                        "general",
                        "please wait..., it could take minutes to summarize a large pull request"
                            .to_string(),
                    );
                } else if count_down == 1 {
                    send_message_to_channel("ik8", "general", r.choice);
                }
                count_down -= 1;
            }
        }
    }
}

pub fn chunk_input(inp: String) -> Vec<String> {
    let mut res = Vec::<String>::new();

    let mut chunks = inp.split_inclusive("diff --git").collect::<Vec<&str>>();
    chunks.remove(0); // Remove the first empty element

    for chunk in chunks {
        let chunk = chunk.lines().take(210).collect::<Vec<&str>>().join("\n");
        res.push(chunk);
    }
    res
}

pub fn serial_prompts(inp_vec: Vec<String>, prompt_start: String) -> Vec<String> {
    use std::collections::VecDeque;
    let mut prompt_vec = Vec::new();
    let mut deq = inp_vec.into_iter().collect::<VecDeque<String>>();

    let start_text = deq.pop_front().unwrap();

    let last_text = deq.pop_back().unwrap();

    let start_prompt = format!("I have a long GitHub pull request diff document in plain text and will process it in smaller chunks. Process chunk 1 and provide a short summary: {start_text}");
    let last_prompt = format!("This is part of the code diff file of a GitHub pull request, please process this chunk and provide a short summary: {last_text}");
    let end_prompt = format!("Finally, these chunks just proccessed are the core data of a pull request on GitHub, {prompt_start}, please summarize the pull request into key points by order of importance");

    match deq.len() {
        0 => {
            let middle_prompt = last_prompt.clone();

            return vec![end_prompt, middle_prompt, start_prompt];
        }
        1.. => {
            while let Some(diff_as_text) = deq.pop_back() {
                let mid_prompt = format!(
                    "Please process this chunk and provide a short summary: {diff_as_text}"
                );

                prompt_vec.push(mid_prompt);
            }
            prompt_vec.insert(0, end_prompt);
            prompt_vec.push(start_prompt);
            return prompt_vec;
        }
        _ => {
            unreachable!()
        }
    };
}
