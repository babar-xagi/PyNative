use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "pynative-native-cli")]
#[command(about = "Native helper CLI for PyNative UI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Doctor => {
            let info = pynative_core::runtime_info();
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
    }

    Ok(())
}
