<div align="right">

[English](README.md)

</div>

# <p align="center">ChatGPT/4 加成的 Github PR 检查机器人</p>

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

[部署此函数到 flows.network](#deploy-your-own-code-review-bot-in-3-simple-steps)，你将获得一个 GitHub 机器人来检查代码和总结拉取请求。它可以帮助忙碌的开源贡献者更快地理解并对 PR 采取行动!下面是一些示例! 

* [[Rust] 支持 WasmEdge Rust SDK 中的主机函数](https://github.com/WasmEdge/WasmEdge/pull/2394#issuecomment-1497819842)
* [[bash] 支持 WasmEdge 安装程序中的 ARM 体系结构](https://github.com/WasmEdge/WasmEdge/pull/1084#issuecomment-1497830324)
* [[C++] 为 WasmEdge 添加 eBPF 插件](https://github.com/WasmEdge/WasmEdge/pull/2314#issuecomment-1497861516)
* [[Haskell] 优化 WasmEdge Component Model 工具的 CLI 实用程序](https://github.com/second-state/witc/pull/73#issuecomment-1507539260)

> 还没被惊艳到吗？[请见此处 bot 指出的“潜在问题1”](https://github.com/second-state/wasmedge-quickjs/pull/82#issuecomment-1498299630)，它识别出了 Rust 算法的低效实现。🤯 

这个机器人会总结 PR 中提交的信息。或者，可以使用[这个机器人](https://github.com/flows-network/github-pr-review)来检查PR中更改的文件。


## 如何工作

当在指定的 GitHub repo 中创建新的 PR 时，此 flow 函数（或🤖）将被触发。 flow 函数会收集 PR 中的内容，并请求 ChatGPT/4 进行检查和总结。结果会作为评论发布回 PR。flow 函数是用Rust编写的，并在[WasmEdge 运行时](https://github.com/wasmedge)上在托管的[flows.network](https://flows.network/)中运行。

* 每次将新的提交推送到此 PR 时，都会自动更新代码检查评论。
* 当有人在 PR 的注释部分中说出一个魔法*触发词*时，可以触发新的代码检查。默认的触发词是"flows summarize"。

## 简单3步部署自己的代码检查机器人

1. 从模板创建一个机器人
2. 配置机器人以检查指定 GitHub repo 中的PR
3. 授权[flows.network](https://flows.network/)访问 GitHub repo


### 0 先决条件

需要使用自己的 [OpenAI API 密钥](https://openai.com/blog/openai-api)。如果还没有注册，请[在此处注册](https://platform.openai.com/signup)。

还需要使用 GitHub 帐户登录 [flows.network](https://flows.network/)。这是免费的。

### 1 从模板创建机器人

### [单击此处](https://flows.network/flow/createByTemplate/summarize-github-pull-requests)

### 2 配置机器人

* `github_owner`：你想在其上部署🤖 的 GitHub repo 的组织。
* `github_repo`：你想在其上部署🤖 的 GitHub repo。

>让我们看一个示例。你想要部署机器人从而总结 `WasmEdge/wasmedge_hyper_demo` repo 中的 PR。这里 `github_owner = WasmEdge`，`github_repo = wasmedge_hyper_demo`。

单击“创建和部署（Create and deploy）”按钮。

## 3 授权访问

接下来，[flows.network](https://flows.network/) 将引导你配置你的 flow 函数 🤖 所需的外部服务。

<img width="927" alt="image" src="https://user-images.githubusercontent.com/45785633/229329158-5ba162a6-1f06-4851-ad46-583840dd6891.png">
对于此 flow 函数，我们需要配置两个 integration。
单击“连接”或“+ 添加新的身份验证”按钮以添加你的 OpenAI API 密钥。
<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">
单击“连接”或“+ 添加新的身份验证”按钮，以使函数可以访问 GitHub repo 以部署🤖。你将被重定向到一个新页面，需要给 [flows.network](https://flows.network/) 授予访问 repo 的权限。

之后，单击“检查”按钮以转到 flow 详情页面。一旦 flow 的状态变为 `running`，PR 摘要 GitHub 机器人就可以开始进行代码检查。每当有新的 PR 和新的 commit ，以及在 PR 评论中出现的魔法词（即`trigger_phrase`）时，这个机器人就会被调用。


<img width="1148" alt="image" src="https://user-images.githubusercontent.com/45785633/229329247-16273aec-f89b-4375-bf2b-4ffce5e35a33.png">


## 常见问题解答

### 自定义机器人

机器人的源代码可在你从模板克隆的 GitHub repo 中找到。请根据自己的需求任意更改源代码（例如，模型、上下文长度、API 密钥和提示）。如果需要帮助，请在 [Discord 中询问](https://discord.gg/ccZn9ZMfFf)！


### 使用 GPT4

默认情况下，该机器人使用 GPT3.5 进行代码审核。如果你的 OpenAI API 密钥可以访问 GPT4，则可以在克隆的源代码 repo 中打开 `src/github-pr-review.rs` 文件，并在源代码中将 `GPT35Turbo` 更改为 `GPT4`。将更改提交并推送回 GitHub。flows.network 平台将自动检测并从你的更新的源代码重建机器人。


### 在多个 repo 上使用机器人

你可以创建一个新的 flow，并导入机器人的源代码 repo（即你从模板克隆的 repo）。然后，可以使用 flow config 来指定 `github_owner` 和 `github_repo`，以指向你需要在其上部署机器人的目标 repo。部署并授权访问该目标 repo。

可以把它安装在你想要部署此机器人的所有目标 repo 上。


### 定制自己的魔法词

进入机器人正在运行的 flow 函数的 "Settings" 选项卡，你可以更新 `trigger_phrase` 配置。该配置的值是让用户触发机器人的魔法词，可以从 PR 评论触发检查。


## 鸣谢

此 flow 函数最初由 [Jay Chen](https://github.com/jaykchen) 创建，[jinser](https://github.com/jetjinser) 为优化来自 GitHub 的事件触发器做出了重大贡献。



<p align="center">
<a href="https://www.producthunt.com/posts/gpt-nitro-for-github-pr?utm_source=badge-featured&utm_medium=badge&utm_souce=badge-gpt&#0045;nitro&#0045;for&#0045;github&#0045;pr" target="_blank"><img src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=387993&theme=light" alt="GPT&#0032;Nitro&#0032;for&#0032;Github&#0032;PR - A&#0032;ChatGPT&#0045;based&#0032;reviewer&#0032;for&#0032;your&#0032;GitHub&#0032;pull&#0032;requests | Product Hunt" style="width: 250px; height: 54px;" width="250" height="54" /></a>
</p>
