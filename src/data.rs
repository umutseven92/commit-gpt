use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The prompt to use when generating the commit message.
    #[arg(
    short,
    long,
    default_value = "Generate a one line Git commit message based on the following diff:"
    )]
    pub prompt: String,

    /// Which GPT model to use. Please note that to use GPT 4, you need to be a Plus subscriber.
    #[arg(short, long, value_enum, default_value_t = Model::GPT35)]
    pub model: Model,

    /// The maximum amount of tokens to generate. Tokens as pieces of words, where 1,000 tokens is about 750 words.
    #[arg(short, long, default_value_t = 512)]
    pub tokens: u16,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Model {
    GPT4,
    GPT35,
}

impl Model {
    pub fn model_str(&self) -> String {
        match self {
            Model::GPT4 => String::from("gpt-4"),
            Model::GPT35 => String::from("gpt-3.5-turbo"),
        }
    }

    pub fn max_tokens(&self) -> usize {
        match self {
            Model::GPT4 => 8192,
            Model::GPT35 => 4096
        }
    }
}
