# <p align="center">Github Pull Request review & summary 🤖 using ChatGPT/4</p>

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

[Deploy this function on flows.network](#deploy-the-pr-summary-app-on-your-github-repo), and you will get a GitHub bot that uses ChatGPT to summary a GitHub Pull Requests automatically. It helps maintainers understand what does this PR do!

See a demo [here](https://github.com/juntao/test/pull/21/)

## How it works

This function will be triggered when a contributor submitted a pull request to the GitHub Repo where you deploy this app and when you mention the keywords you set up in the PR's comment zone.


## Prerequisite 

You will need an [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

## Deploy the PR summary app on your GitHub repo

To install the PR summary GitHub App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/github-pr-summary/) and go to flows.network to deploy your function. 

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Create a Flow" button to start deploying the PR Summary APP
3. Authenticate the [flows.network](https://flows.network/) to access the `github-pr-summary` repo you just forked. 
<img width="950" alt="image" src="https://user-images.githubusercontent.com/45785633/229329081-93728947-ad9f-44fb-85b1-067e6a0eb8ac.png">

4. Click on the Advanced text and you will see more settings. we can fill in the required Environment Variables here. In this example, we have four variables. 
* One is `login`: Fill in your personel github id here. The github app will act as you when respond to questions. 
* The second one is `owner`: Fill in the GitHub org Name you want to connect here. 
* The thrid one is `repo` : Fill in the GitHub repo Name under the GitHub org you just entered. 
* The last one is `openai_key_name`: **Fill in the name you want to name your OpenAI Key**.
* The last one is `trigger_phrase`: Fill in the keywords to trigger this function again.

<img width="886" alt="image" src="https://user-images.githubusercontent.com/45785633/229329142-b7d77e53-4f3a-4d87-9136-4216191b18fc.png">

5. At last, click the Deploy button to deploy your function.

### Configure SaaS integrations

After that, the flows.network will direct you to configure the SaaS integration required by your flow.

<img width="927" alt="image" src="https://user-images.githubusercontent.com/45785633/229329158-5ba162a6-1f06-4851-ad46-583840dd6891.png">


Here we can see, we need to configue two SaaS integrations.

1. Click on the "Connect/+ Add new authentication" button to authenticate your OpenAI account. You'll be redirected to a new page where you could copy and paste your OpenAI API key and then name the key. **Note that the name you enter here should be the same as the name in the environment variables.**

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">

2. Click the "Connect/+ Add new authentication" button to authenticate your GitHub account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a repo where you want to deploy this app. This repo is the one you entered into the environment variables above.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, the PR summary GitHub App goes live. Go ahead and let ChatGPT help you understand the PRs.

<img width="1148" alt="image" src="https://user-images.githubusercontent.com/45785633/229329247-16273aec-f89b-4375-bf2b-4ffce5e35a33.png">

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!


## Credits

This repo is originally contributed by Jay Chen.
