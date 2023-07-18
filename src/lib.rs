use std::error::Error;
use std::process::Command;

use async_openai::{
    Client,
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::data::Model;

pub mod data;

fn get_diff() -> Result<String, Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "git diff --cached"])
            .output()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git diff --cached")
            .output()?
    };

    if !output.status.success() {
        Err("Could not run git diff. Please make sure you are in a valid git repository.")?
    }
    Ok(String::from_utf8(output.stdout).unwrap())
}

async fn generate_commit_message(
    diff: String,
    prompt: String,
    model: Model,
    max_tokens: u16,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let prompt = format!("{} {}", prompt, diff);

    check_prompt_length(&prompt, &model)?;

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(max_tokens)
        .model(model.model_str())
        .messages([ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(prompt)
            .build()?])
        .build()?;

    let response = client.chat().create(request).await?;

    let result = response.choices[0].message.content.clone().unwrap();

    Ok(result)
}

/// Roughly calculates the token size of the prompt, taken from [OpenAI help page regarding tokens](https://help.openai.com/en/articles/4936856-what-are-tokens-and-how-to-count-them).
/// As stated in the the above link, > 1 token ~= 4 chars in English
fn check_prompt_length(prompt: &String, model: &Model) -> Result<(), Box<dyn Error>> {
    let max_tokens = model.max_tokens();
    let prompt_token_size = prompt.len() / 4;

    if prompt_token_size > max_tokens {
        Err(format!("Prompt token size (prompt + diff length) {} is bigger than {} max token size of {}. Please try with a smaller diff.", prompt_token_size, model.model_str(), max_tokens))?
    }

    Ok(())
}

fn copy_message_to_clipboard(message: &String) -> Result<(), Box<dyn Error>> {
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(message.to_owned())?;

    Ok(())
}

pub async fn run(
    prompt: String,
    model: Model,
    max_tokens: u16,
) -> Result<Option<String>, Box<dyn Error>> {
    let diff = get_diff()?;
    if diff == "" {
        Ok(None)
    } else {
        match generate_commit_message(diff, prompt, model, max_tokens).await {
            Ok(x) => {
                copy_message_to_clipboard(&x).unwrap_or_else(|err| {
                    eprintln!("Could not copy commit message to clipboard: {err}")
                });
                Ok(Some(x))
            }
            Err(e) => Err(e),
        }
    }
}
