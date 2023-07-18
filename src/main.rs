use std::error::Error;

use clap::Parser;

use commit_gpt::data::Args;
use commit_gpt::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match run(args.prompt, args.model, args.tokens).await? {
        None => {
            println!("No diff found.");
        }
        Some(commit_message) => {
            println!("COMMIT MESSAGE: {commit_message}");
        }
    }
    Ok(())
}
