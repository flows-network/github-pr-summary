“# <p align="center">ChatGPT/4 代码检查机器人</p>

<p align="center"> <a href="https://discord.gg/ccZn9ZMfFf"> <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord"> </a> <a href="https://twitter.com/flows_network"> <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter"> </a> <a href="https://flows.network/flow/createByTemplate/summarize-github-pull-requests"> <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow"> </a> </p>
部署此函数到flows.network，您将获得一个GitHub机器人来检查代码和总结拉取请求。它可以帮助忙碌的开源贡献者更快地理解和作出对PR的决定!下面是一些示例!

[Rust] 支持WasmEdge Rust SDK中的主机函数
[bash] 支持WasmEdge安装程序中的ARM体系结构
[C++] 为WasmEdge添加eBPF插件
[Haskell] 改进WasmEdge组件模型工具的CLI实用程序

还有疑虑? 请见本评论中的“潜在问题1”，它识别出Rust算法的低效实现。🤯

此机器人总结PR中的提交。或者您可以使用此机器人审查PR中的更改文件。

工作原理
此流函数(或机器人)将在指定的GitHub仓库中提出新PR时触发。流函数收集PR中的内容，并要求ChatGPT/4对其进行检查和总结。然后，结果将作为评论发布回PR。流函数是用Rust编写的，并在flows.network上的托管WasmEdge Runtime上运行。

代码检查评论每次将PR推送到这项业务时都会自动更新。
当有人在PR的评论部分说出魔法触发短语时，可以触发新的代码检查。默认的触发短语是“flows summarize”。

在3个简单步骤中部署您自己的代码检查机器人

从模板创建机器人
只需点击这里

配置机器人
github_owner: 您要部署机器人的GitHub组织。
github_repo: 您要部署机器人的GitHub仓库。
举个例子。您想要部署机器人来总结WasmEdge/wasmedge_hyper_demo仓库的PR。这里github_owner = WasmEdge和github_repo = wasmedge_hyper_demo。

点击“创建和部署”按钮。

授权访问
之后，flows.network会将您重定向到配置流函数机器人所需的外部服务。
<img width="927" alt="image" src="https://user-images.githubusercontent.com/45785633/229329158-5ba162a6-1f06-4851-ad46-583840dd6891.png"> 对于此流函数，我们需要配置两个集成。
单击“连接”或“+ 添加新身份验证”按钮添加OpenAI API密钥。

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png"> 单击“连接”或“+ 添加新身份验证”按钮授予函数访问权限GitHub仓库部署机器人。 您将被重定向到一个新页面,在其中必须授予flows.network对仓库的权限。
之后，单击“检查”按钮转到流详情页面。一旦流的状态变为运行，PR摘要GitHub机器人就准备提供代码检查!该机器人在每次新的PR、每次新的提交以及PR评论中的魔法词(即触发词)所召唤。

<img width="1148" alt="image" src="https://user-images.githubusercontent.com/45785633/229329247-16273aec-f89b-4375-bf2b-4ffce5e35a33.png">
常见问题解答

定制机器人
机器人的源代码可在您从模板克隆的GitHub仓库中获得。随意更改源代码(例如，模型、上下文长度、API密钥和提示)以满足您自己的需求。如果您需要帮助，请在Discord上提问!

使用GPT4
默认情况下，该机器人使用GPT3.5进行代码检查。如果您的OpenAI API密钥可以访问GPT4，您可以打开您克隆的源代码仓库中的src/github-pr-review.rs文件，并在源代码中将GPT35Turbo更改为GPT4。提交并推送更改回GitHub。
flows.network平台会自动检测并从您更新的源代码重建机器人。

在多个仓库上使用机器人
您可以创建新的流程并导入机器人的源代码仓库(即您从模板克隆的仓库)。然后，您可以使用流配置指定github_owner和github_repo指向需要部署机器人的目标仓库。部署并授权访问该目标仓库。

您可以对需要部署此机器人的所有目标仓库重复此操作。

更改魔法短语
转到运行流函数机器人的“设置”选项卡，您可以更新trigger_phrase配置。此配置的值是用户将在PR评论中说出以触发审阅的魔法短语。

致谢
此流函数最初由Jay Chen创建，jinser对优化来自GitHub的事件触发器作出了重大贡献。

<p align="center"> <a href="https://www.producthunt.com/posts/gpt-nitro-for-github-pr?utm_source=badge-featured&utm_medium=badge&utm_souce=badge-gpt&#0045;nitro&#0045;for&#0045;github&#0045;pr" target="_blank"><img src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=387993&theme=light" alt="GPT&#0032;Nitro&#0032;for&#0032;Github&#0032;PR - A&#0032;ChatGPT&#0045;based&#0032;reviewer&#0032;for&#0032;your&#0032;GitHub&#0032;pull&#0032;requests | Product Hunt" style="width: 250px; height: 54px;" width="250" height="54" /></a> </p>
