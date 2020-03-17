<div align="center">
    <h1>kiwiyou-bot</h1>
    <a href="https://travis-ci.com/github/kiwiyou/kiwiyou-bot">
        <img src="https://travis-ci.com/kiwiyou/kiwiyou-bot.svg?branch=master">
    </a>
    <p>Rusty telegram bot built on serverless platform</p>
</div>

## Table of contents

- [Commands](#commands)
- [How to build](#how-to-build)
- [Credits](#credits)

## Commands

All commands are available on [@kiwiyou_bot](https://t.me/kiwiyou_bot)

- `/lang <language>`
  - Set the bot's language for the current chat.
- `/usearch <keyword>`
  - Look up a unicode character whose name contains the keyword.

## How to build

- Prerequisites
  - [npm](https://www.npmjs.com/)
  - [serverless framework](https://serverless.com/)
  - Rust 1.41.0 or higher
  - Amazon AWS Account
  - A telegram bot account (can be created by talking to [@BotFather](https://t.me/BotFather))

1. Clone the repository:

```bash
git clone https://github.com/kiwiyou/kiwiyou-bot.git
```

2. Configure your AWS Systems Manager to have the bot's token

- In the Parameter Store, You should have both `/kiwiyou-bot/token/head` and `/kiwiyou-bot/token/body`.
- Given the bot's token, for example, say 01234567890:ABCDEFGHIJKLMabcdefghijklm123456789:
  - `/kiwiyou-bot/token/head` should be `01234567890`.
  - `/kiwiyou-bot/token/body` should be `ABCDEFGHIJKLMabcdefghijklm123456789`.

3. Edit `serverless.yml` to match with your bot's settings.

```yaml
BOT_NAME: '@your_bot'
```

4. Deploy your bot:

```bash
serverless deploy

# Or, if you have the npm version:
npx serverless deploy
```

It will give you an endpoint URL like:

```bash
endpoints:
  POST - https://example123.execute-api.eu-west-2.amazonaws.com/dev/<your-token-head>/<your-token-body>
```

5. Setup a webhook for your bot by visiting:

`https://api.telegram.org/bot<your-bot-token>/setWebhook?url=<your-endpoint-url>`

*Done!* You can enjoy your own bot.

## Credits

- [teloxide](https://github.com/teloxide/teloxide)
- [serverless-aws-rust-http](https://github.com/softprops/serverless-aws-rust-http)
