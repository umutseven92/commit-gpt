
# Commit GPT

Generates commit messages for your staged (but unpushed) commits using ChatGPT.

## Prerequisites

1. Create an account & [generate an API key for OpenAI](https://platform.openai.com/account/api-keys).
2. Add your API key as an environment variable:
   ```bash
    export OPENAI_API_KEY='sk-...'
    ```
 
## Installation

```bash
$ cargo install commit-gpt
```

## Usage

```bash
-- Stage your changes
$ git add -A
$ commit-gpt
-- Commit message will be printed, as such:
$ COMMIT MESSAGE: Update README.md with instructions..
-- It will also be copied to your clipboard, so you can just paste.
$ git commit -m "..."
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

## Tweaking the Prompt

I suggest you tweak the prompt, depending on your needs. The default prompt is fine for simple commits, but for bigger & more important ones, you can improve the commit messages greatly with some simple tweaks.

Some prompts and their results:

* `Generate a Git commit message based on the following diff:` (default prompt)

```
COMMIT MESSAGE: "Add new testing configuration and invert_color function in shade_generator module"
```

* `Generate a Git commit message, giving context to the changes, based on the following diff:`

```
COMMIT MESSAGE: "Added testing environment configuration and color inversion function
This update involves two key changes to the shade_generator. Firstly, a config.toml file was added with the inclusion of a testing environment attribute set to true. This initiates the setup for testing and debugging of shade_generator functions. Secondly, color inversion functionality is now introduced to the generator.py script. This new method, invert_color, will allow the inversion of given color, which can improve color efficiency and variety in the shade generation process."
```

* `Generate a Git commit message, giving as much detail as you can, based on the following diff:`

```
COMMIT MESSAGE: "Added testing environment variable and color inversion function

This commit introduces a couple of significant changes. 
Firstly, a new file named config.toml has been added to the shade_generator directory. This file contains a new environment variable setting (env), defined for testing purposes. This suggests that any code that uses this environment variable will switch to a testing mode when the 'testing' variable is set to true. 
Secondly, in the generator.py file within the shade_generator directory, a new function named 'invert_color' has been added. While the implementation of this function is not complete in the diff provided, it seems like it will take a color and output another color, presumably being the inverse of the input. Two new variables, avc and vde, have been declared in this function, but their purpose is not explicit from the diff.
This commit is intended to enhance the functionality of the shade_generator tool by adding a potential for testing and the starting functionality for color inversion.

```

* `Generate a Git commit message in the style of a 15th century poem, based on the following diff:`

```
COMMIT MESSAGE: "From nought to song, config.toml now exists,
With [env] testing = true midst its lists.
Shade generator, too, hath been revised,
An invert color method is devised.
True work is done, yet not with testing ends,
The cause of hues to 'gainst their nature bend.
Doth make our colors dance a contrary turn,
Appear they will as invert, concern.
To the labor of scripting, I hath thee invite,
Run the tests, ensure all execute right."
```

* `Generate a Git commit message in the style of Linus Torvalds, based on the following diff:`

```
COMMIT MESSAGE: "Commit new test environment config and add color inversion function
I've added a file for us to manage our config settings in, with a testing flag to boot. 
But wait, there's more! It seems that the shade generator was seriously lacking some functionality. Lo and behold, the invert_color function. I bet you didn't think we'd need that! Now color inversion is as easy as calling a function. 
Don't ask me what those magic numbers are, they're probably very important."
```