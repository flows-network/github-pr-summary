# <p align="center">ChatGPT/4 code reviewer for Github PR</p>

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

[Deploy this function on flows.network](#deploy-the-pr-summary-app-for-your-github-repo), and you will get a GitHub 🤖 to review and summarize Pull Requests. It helps busy open source contributors understand and make decisions on PRs faster! A few examples below!

* [[Rust] Improve support for host functions in the WasmEdge Rust SDK](https://github.com/WasmEdge/WasmEdge/pull/2394#issuecomment-1497819842)
* [[bash] Support ARM architecture in the WasmEdge installer](https://github.com/WasmEdge/WasmEdge/pull/1084#issuecomment-1497830324)
* [[C++] Add an eBPF plugin for WasmEdge](https://github.com/WasmEdge/WasmEdge/pull/2314#issuecomment-1497861516)
* [[Haskell] Improve the CLI utility for WasmEdge Component Model tooling](https://github.com/second-state/witc/pull/73#issuecomment-1507539260)

> Still not convinced? [See "potential problems 1" in this review](https://github.com/second-state/wasmedge-quickjs/pull/82#issuecomment-1498299630), it identified an inefficient Rust implementation of an algorithm. 🤯

This bot summarizes commits in the PR. Alternatively, you can use [this bot](https://github.com/flows-network/github-pr-review) to review changed files in the PR. 

## How it works

This flow function (or 🤖) will be triggered when a new PR is raised in the designated GitHub repo. The flow function collects the content in the PR, and asks ChatGPT/4 to review and summarize it. The result is then posted back to the PR as a comment. The flow functions are written in Rust and run in hosted [WasmEdge Runtimes](https://github.com/wasmedge) on [flows.network](https://flows.network/).

* The code review comment is updated automatically every time a new commit is pushed to this PR.
* A new code review could be triggered when someone says a magic *trigger phrase* in the PR's comments section. The default trigger phrase is "flows summarize".

### Deploy your own code review bot in 3 simple steps

1. Fork this repo to your own GitHub account. It contains the source code for the GitHub bot.
2. Import the forked repo on [flows.network](https://flows.network/) as a flow function.
3. Connect the flow function to the GitHub repo you want to deploy the bot on (using the [flows.network](https://flows.network/) UI). 

<p align="center">
  <a href="https://youtu.be/kvBhNBXmBaE" taregt=_blank><img src="https://img.youtube.com/vi/kvBhNBXmBaE/hqdefault.jpg"/></a><br/>
  <i>Click on the picture above to watch a 3-min tutorial video</i>
</p>

## Prerequisites

You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

You will also need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

## Deploy the PR review 🤖 onto your GitHub repos

The 🤖 is designed to run on [flows.network](https://flows.network/), a serverless platform for SaaS and AI automation.

### 1 Fork this repo

Fork [this repo](https://github.com/flows-network/github-pr-summary/) into your own GitHub account.

> If your OpenAI API key has GPT4 access, you can change `GPT35Turbo` to `GPT4` in your fork of the source code. GPT4 provides substantially better code reviews, but it is also 10x more expensive.

### 2 Deploy the bot's source code on flow.network

Go to [flows.network](https://flows.network/) to deploy your own flow function (🤖) from the forked source code.

1. Click on the "Create a Flow" button to start.
2. Authenticate the [flows.network](https://flows.network/) to access the `github-pr-summary` repo you just forked. **NOTE: This is NOT the repo you want to install the bot on.**

<img width="950" alt="image" src="https://user-images.githubusercontent.com/45785633/229329081-93728947-ad9f-44fb-85b1-067e6a0eb8ac.png">

3. Click on the "Advanced" link to see more settings. Fill in the following environment variables. 

> The 4 variables below are defined in the flow function's Rust source code. You can assign their values in the source code in your fork directly and skip the steps below.

* `login`: Your personal GitHub id. The GitHub app will act as you when posting reviews.
* `owner`: GitHub org for the repo *you want to deploy the 🤖 on*.
* `repo` : GitHub repo *you want to deploy the 🤖 on*.
* `trigger_phrase`: Optional -- The magic phrase to trigger a review from a PR comment.

> Let's see an example. You forked the flow function source code to `my-name/github-pr-summary` and would like to deploy the bot to summarize PRs on `my-company/work-project` repo. Here `login = my-name`, `owner = my-company` and `repo = work-project`.

![image](https://user-images.githubusercontent.com/45785633/234774015-0a8de4be-17fd-4fe2-be31-ee7ed0e9b462.png)

4. Click on the Deploy button.

### 3 Configure integrations

After that, [flows.network](https://flows.network/) will direct you to configure the external services required by your flow function 🤖.

<img width="927" alt="image" src="https://user-images.githubusercontent.com/45785633/229329158-5ba162a6-1f06-4851-ad46-583840dd6891.png">

For this flow function, we need to configue two integrations.

1. Click on the "Connect" or "+ Add new authentication" button to add your OpenAI API key.

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">

2. Click on the "Connect" or "+ Add new authentication" button to give the function access to the GitHub repo to deploy the 🤖. That is to give access to the `owner/repo` in the environment variables. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to the repo.

After that, click on the "Check" button to go to the flow details page. As soon as the flow's status became `running`, the PR summary GitHub bot is ready to give code reviews! The bot is summoned by every new PR or magic words (i.e., `trigger_phrase`) in PR comments.

<img width="1148" alt="image" src="https://user-images.githubusercontent.com/45785633/229329247-16273aec-f89b-4375-bf2b-4ffce5e35a33.png">

## Credits

This flow function is originally created by [Jay Chen](https://github.com/jaykchen), and [jinser](https://github.com/jetjinser) made significant contributions to optimize the event triggers from GitHub.

<p align="center">
<a href="https://www.producthunt.com/posts/gpt-nitro-for-github-pr?utm_source=badge-featured&utm_medium=badge&utm_souce=badge-gpt&#0045;nitro&#0045;for&#0045;github&#0045;pr" target="_blank"><img src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=387993&theme=light" alt="GPT&#0032;Nitro&#0032;for&#0032;Github&#0032;PR - A&#0032;ChatGPT&#0045;based&#0032;reviewer&#0032;for&#0032;your&#0032;GitHub&#0032;pull&#0032;requests | Product Hunt" style="width: 250px; height: 54px;" width="250" height="54" /></a>
</p>
