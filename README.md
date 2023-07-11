<div align="right">

  [中文文档](README-zh.md)

</div>

# <p align="center">ChatGPT/4 code reviewer for Github PR</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/summarize-github-pull-requests">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](#deploy-your-own-code-review-bot-in-3-simple-steps), and you will get a GitHub 🤖 to review and summarize Pull Requests. It helps busy open source contributors understand and make decisions on PRs faster! A few examples below!

* [[Rust] Improve support for host functions in the WasmEdge Rust SDK](https://github.com/WasmEdge/WasmEdge/pull/2394#issuecomment-1497819842)
* [[bash] Support ARM architecture in the WasmEdge installer](https://github.com/WasmEdge/WasmEdge/pull/1084#issuecomment-1497830324)
* [[C++] Add an eBPF plugin for WasmEdge](https://github.com/WasmEdge/WasmEdge/pull/2314#issuecomment-1497861516)
* [[Haskell] Improve the CLI utility for WasmEdge Component Model tooling](https://github.com/second-state/witc/pull/73#issuecomment-1507539260)

> Still not convinced? [See "potential problems 1" in this review](https://github.com/second-state/wasmedge-quickjs/pull/82#issuecomment-1498299630), it identified an inefficient Rust implementation of an algorithm. 🤯

This bot **summarizes commits in the PR**. Alternatively, you can use [this bot](https://github.com/flows-network/github-pr-review) to review changed files in the PR. 

## How it works

This flow function (or 🤖) will be triggered when a new PR is raised in the designated GitHub repo. The flow function collects the content in the PR, and asks ChatGPT/4 to review and summarize it. The result is then posted back to the PR as a comment. The flow functions are written in Rust and run in hosted [WasmEdge Runtimes](https://github.com/wasmedge) on [flows.network](https://flows.network/).

* The code review comment is updated automatically every time a new commit is pushed to this PR.
* A new code review could be triggered when someone says a magic *trigger phrase* in the PR's comments section. The default trigger phrase is "flows summarize".

## Deploy your own code review bot in 3 simple steps

1. Create a bot from a template
2. Add your OpenAI API key
3. Configure the bot to review PRs on a specified GitHub repo

### 0 Prerequisites

You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

You will also need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

### 1 Create a bot from a template

[**Just click here**](https://flows.network/flow/createByTemplate/Summarize-Pull-Request)

Review the `trigger_phrase` variable. It is the magic words you type in a PR comment to manually summon the review bot.

Click on the **Create and Build** button.

### 2 Add your OpenAI API key

You will now set up OpenAI integration. Click on **Connect**, enter your key and give it a name.

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

Close the tab and go back to the flow.network page once you are done. Click on **Continue**.

### 3 Configure the bot to access GitHub

Next, you will tell the bot which GitHub repo it needs to monitor for upcoming PRs to review.

* `github_owner`: GitHub org for the repo *you want to deploy the 🤖 on*.
* `github_repo` : GitHub repo *you want to deploy the 🤖 on*.

> Let's see an example. You would like to deploy the bot to review code in PRs on `WasmEdge/wasmedge_hyper_demo` repo. Here `github_owner = WasmEdge` and `github_repo = wasmedge_hyper_demo`.

Click on the **Connect** or **+ Add new authentication** button to give the function access to the GitHub repo to deploy the 🤖. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to the repo.

[<img width="450" alt="image" src="https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473">](https://github.com/flows-network/github-pr-summary/assets/45785633/6cefff19-9eeb-4533-a20b-03c6a9c89473)

Close the tab and go back to the flow.network page once you are done. Click on **Deploy**.

### Wait for the magic!

This is it! You are now on the flow details page waiting for the flow function to build. As soon as the flow's status became `running`, the bot is ready to give code reviews! The bot is summoned by every new PR, every new commit, as well as magic words (i.e., `trigger_phrase`) in PR comments.

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/229329247-16273aec-f89b-4375-bf2b-4ffce5e35a33.png">](https://user-images.githubusercontent.com/45785633/229329247-16273aec-f89b-4375-bf2b-4ffce5e35a33.png)

## FAQ

### Customize the bot

The bot's source code is available in the GitHub repo you cloned from the template. Feel free to make changes to the source code (e.g., model, context length, API key and prompts) to fit your own needs. If you need help, [ask in Discord](https://discord.gg/ccZn9ZMfFf)!

### Use GPT4

By default, the bot uses GPT3.5 for code review. If your OpenAI API key has access to GPT4, you can open the `src/github-pr-review.rs` file
in your cloned source code repo, and change `GPT35Turbo` to `GPT4` in the source code. Commit and push the change back to GitHub.
The flows.network platform will automatically detect and rebuild the bot from your updated source code.

### Use the bot on multiple repos

You can [manually create a new flow](https://flows.network/flow/new) and import the source code repo for the bot (i.e., the repo you cloned from the template). Then, you can use the flow config to specify the `github_owner` and `github_repo` to point to the target repo you need to deploy the bot on. Deploy and authorize access to that target repo.

You can repeat this for all target repos you would like to deploy this bot on.

> You could have a single flow function repo deployed as the source code for multiple bots. When you update the source code in the repo, and push it to GitHub, it will change the behavior of all the bots.

### Change the magic phrase

Go to the "Settings" tab of the running flow function for the bot, you can update the `trigger_phrase` config. The value of this config is the magic phrase the user will say to trigger a review from a PR comment.

## Credits

This flow function is originally created by [Jay Chen](https://github.com/jaykchen), and [jinser](https://github.com/jetjinser) made significant contributions to optimize the event triggers from GitHub.

<p align="center">
<a href="https://www.producthunt.com/posts/gpt-nitro-for-github-pr?utm_source=badge-featured&utm_medium=badge&utm_souce=badge-gpt&#0045;nitro&#0045;for&#0045;github&#0045;pr" target="_blank"><img src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=387993&theme=light" alt="GPT&#0032;Nitro&#0032;for&#0032;Github&#0032;PR - A&#0032;ChatGPT&#0045;based&#0032;reviewer&#0032;for&#0032;your&#0032;GitHub&#0032;pull&#0032;requests | Product Hunt" style="width: 250px; height: 54px;" width="250" height="54" /></a>
</p>
