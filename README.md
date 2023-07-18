
# Commit GPT

Generates commit messages for your staged (but unpushed) commits using ChatGPT.

## Prerequisites

1. Create an account & [generate an API key for OpenAI](https://platform.openai.com/account/api-keys).
2. Add your API key as an environment variable:
   ```bash
    export OPENAI_API_KEY='sk-...'
    ```
   
## Usage

```bash
-- Stage your changes
git add -A
cargo run
-- Commit message will be printed, as such:
COMMIT MESSAGE: Update README.md with instructions..
-- It will also be copied to your clipboard, so you can just paste.
git commit -m "..."
```

## Options

```
  -p, --prompt <PROMPT>  The prompt to use when generating the commit message [default: "Generate a Git commit message based on the following diff:"]
  -m, --model <MODEL>    Which GPT model to use [default: gpt4] [possible values: gpt4, gpt35]
  -t, --tokens <TOKENS>  The maximum amount of tokens to generate. Tokens as pieces of words, where 1,000 tokens is about 750 words [default: 512]
  -h, --help             Print help
  -V, --version          Print version
```

* GPT 4 works much better than GPT 3.5, so I suggest sticking with the default.
* As you get billed per token, be careful when increasing the `tokens` parameter. I suggest setting a [Usage Limit](https://platform.openai.com/account/billing/limits) to be sure.