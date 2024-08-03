<div align="right">

</div>

# <p align="center">Agent to summarize Github PRs</p>

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

[Deploy this function on flows.network](#deploy-your-own-code-review-bot-in-3-simple-steps), and you will get a GitHub agent to review and summarize Pull Requests. It helps busy open source contributors understand and make decisions on PRs faster! A few examples below!

> We recommend you to use a [GaiaNet node](https://github.com/GaiaNet-AI/gaianet-node) running an open source coding LLM as the backend to perform PR reviews and summarizations. You can use [a community node](https://docs.gaianet.ai/user-guide/nodes#codestral-22b-v01) or run a node [on your own computer](https://github.com/GaiaNet-AI/node-configs/tree/main/codestral-0.1-22b)!

* [[Rust] Improve support for host functions in the WasmEdge Rust SDK](https://github.com/WasmEdge/WasmEdge/pull/2394#issuecomment-1497819842)
* [[bash] Support ARM architecture in the WasmEdge installer](https://github.com/WasmEdge/WasmEdge/pull/1084#issuecomment-1497830324)
* [[C++] Add an eBPF plugin for WasmEdge](https://github.com/WasmEdge/WasmEdge/pull/2314#issuecomment-1497861516)
* [[Haskell] Improve the CLI utility for WasmEdge Component Model tooling](https://github.com/second-state/witc/pull/73#issuecomment-1507539260)

> Still not convinced? [See "potential problems 1" in this review](https://github.com/second-state/wasmedge-quickjs/pull/82#issuecomment-1498299630), it identified an inefficient Rust implementation of an algorithm. 🤯

This bot **summarizes commits in the PR**. Alternatively, you can use [this bot](https://github.com/flows-network/github-pr-review) to review changed files in the PR. 

## How it works

This flow function will be triggered when a new PR is raised in the designated GitHub repo. The flow function collects the content in the PR, and asks ChatGPT/4 to review and summarize it. The result is then posted back to the PR as a comment. The flow functions are written in Rust and run in hosted [WasmEdge Runtimes](https://github.com/wasmedge) on [flows.network](https://flows.network/).

* The PR summary comment is updated automatically every time a new commit is pushed to this PR.
* A new summary could be triggered when someone says a magic *trigger phrase* in the PR's comments section. The default trigger phrase is "flows summarize".

## Deploy your own code review bot in 3 simple steps

1. Create a bot from your own GitHub repo
2. Configure the bot to review PRs
3. Connect to GitHub for access to the target repo

### 0 Prerequisites

You will also need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

### 1 Create a bot from your own GitHub repo

Just fork this repo to your own GitHub account.

Then, from [flows.network](https://flows.network/), you can "Create a Flow" and select your forked repo.
It will create a flow function based on the code in your forked repo.

Click on the "Advanced" button to see configuration options for the flow function.

### 2 Configure the bot to review PRs

<img src="https://raw.githubusercontent.com/flows-network/github-pr-summary/main/images/config.png" width="450"/>

Next, you will tell the bot which GitHub repo it needs to monitor for upcoming PRs to review.

* `github_owner`: GitHub org for the repo *you want to review PRs*
* `github_repo` : GitHub repo *you want to review PRs*
* `trigger_phrase` : The magic words to write in a PR comment to summon the bot. It defaults to "flows summarize".

> Let's see an example. You would like to deploy the bot to review code in PRs on `WasmEdge/wasmedge_hyper_demo` repo. Here `github_owner = WasmEdge` and `github_repo = wasmedge_hyper_demo`.

And the LLM API service you want to use to review the PRs.

* `llm_api_endpoint` : The OpenAI compatible API service endpoint for the LLM to conduct code reviews. We recommend
the [Codetral-0.1-22B GaiaNet node](https://github.com/GaiaNet-AI/node-configs/tree/main/codestral-0.1-22b): `https://codestral-01-22b.us.gaianet.network/v1`
* `llm_model_name` : The model name required by the API service. We recommend the following model name for the above GaiaNet node: `Codestral-22B-v0.1-hf-Q5_K_M`
* `llm_ctx_size` : The context window size of the selected model. The Codestral 0.1 model has a 16k context window, which is `16384`.
* `llm_api_key` : Optional: The API key if required by the LLM service provider. It is not required for the GaiaNet node.

Click on the **Build** button.

### 3 Connect to GitHub for access to the target repo

Finally, the GitHub repo will need to give you access so that the flow function can
access and review its PRs! In this next screen, you will connect to GitHub and authorize access.

Click on the **Connect** or **+ Add new authentication** button to give the function access to the GitHub repo. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to the repo.

<img src="https://raw.githubusercontent.com/flows-network/github-pr-summary/main/images/github_integration.png" width="450"/>

Close the tab and go back to the flow.network page once you are done. Click on **Deploy**.

### Wait for the magic!

This is it! You are now on the flow details page waiting for the flow function to build. As soon as the flow's status became `running`, the bot is ready to give code reviews! The bot is summoned by every new PR, every new commit, as well as magic words (i.e., `trigger_phrase`) in PR comments.

<img src="https://raw.githubusercontent.com/flows-network/github-pr-summary/main/images/success.png" width="450"/>

## FAQ

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

###Adding test compare elements
