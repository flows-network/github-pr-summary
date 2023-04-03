# <p align="center">GitHub Pull Request review & summary 🤖 using ChatGPT/4</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/new">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](#deploy-the-pr-summary-app-for-your-github-repo), and you will get a GitHub bot to review and summarize Pull Requests. It helps busy open source contributors understand and make decisions on PRs faster! A few examples below!

* [[Rust] Improve support for host functions in the WasmEdge Rust SDK](https://github.com/WasmEdge/WasmEdge/pull/2394#issuecomment-1493216830)
* [[Shell] Support ARM architecture in the WasmEdge installer](https://github.com/WasmEdge/WasmEdge/pull/1084#issuecomment-1493413405)

## How it works

This flow function will be triggered and executed when a new PR is raised in a designated GitHub repo. It can also be triggered again when someone says a magic "trigger phrase" in the PR's comments section. Once triggered, the flow function collects the content in the PR, and asks ChatGPT/4 to review and summarize it. The result is then posted back to the PR as a comment.

The GitHub repo is connected to the flow function via the [flows.network](https://flows.network/) platform. The "trigger phrase" can also be configured in flows.network.

> The flow functions are written in Rust and runs in hosted [WasmEdge Runtimes](https://github.com/wasmedge) on flows.network.

## Prerequisites

You will need an [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

You will also need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

## Deploy the PR summary app for your GitHub repo

The app is designed to run on [flows.network](https://flows.network/), a serverless platform for SaaS and AI automations.

### 1 Fork this repo

Fork [this repo](https://github.com/flows-network/github-pr-summary/) into your own GitHub account.

### 2 Deploy the code on flow.network

Go to [flows.network](https://flows.network/) to deploy your own flow function from the forked source code.

1. Click on the "Create a Flow" button to start.
2. Authenticate the [flows.network](https://flows.network/) to access the `github-pr-summary` repo you just forked. 
<img width="950" alt="image" src="https://user-images.githubusercontent.com/45785633/229329081-93728947-ad9f-44fb-85b1-067e6a0eb8ac.png">

3. Click on the "Advanced" link to see more settings. Fill in the following environment variables. 

> The 5 varisbles below are defined in the flow function's Rust source code. You can assign their values in the source code in your fork directly and skip the environment variables setup below.

* `login`: Fill in your personal github id here. The github app will act as you when posting review comments. 
* `owner`: Fill in the GitHub org for the repo you want to deploy the bot on.
* `repo` : Fill in the GitHub repo you want to deploy the bot on.
* `openai_key_name`: **Fill in any name you wish for your OpenAI API key**. We will connect this name to the actual key later.
* `trigger_phrase`: Fill in the magic phrase to trigger a review in an existing PR.

<img width="886" alt="image" src="https://user-images.githubusercontent.com/45785633/229329142-b7d77e53-4f3a-4d87-9136-4216191b18fc.png">

4. At last, click on the Deploy button.

### 3 Configure integrations

After that, the flows.network will direct you to configure the external services required by your flow.

<img width="927" alt="image" src="https://user-images.githubusercontent.com/45785633/229329158-5ba162a6-1f06-4851-ad46-583840dd6891.png">

For this flow function, we need to configue two integrations.

1. Click on the "Connect" or "+ Add new authentication" buttons to add your OpenAI API keys. You could paste your OpenAI API key here and then give it a name. **Note that the name here must match the name in the `openai_key_name` environment variable.**

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">

2. Click the "Connect" or "+ Add new authentication" buttons to give the function access to the GitHub repo to deploy the bot. That is to give access to the `owner/repo` in the environment variables. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission on the repo.

After that, click on the "Check" button to go to the flow details page. As soon as the flow's status became `running`, the PR summary GitHub bot is ready to take new PRs or comments in existing PRs.

<img width="1148" alt="image" src="https://user-images.githubusercontent.com/45785633/229329247-16273aec-f89b-4375-bf2b-4ffce5e35a33.png">

## Credits

This flow function is originally created by [Jay Chen](https://github.com/jaykchen), and [jinser](https://github.com/jetjinser) made significant contributions to optimize the event triggers from GitHub.
