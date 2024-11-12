use anyhow::Result;
use clap::{Parser, Subcommand};
use my_gym_data_rust_parser::parse_exercise_log;
use std::fs;

#[derive(Parser)]
#[command(name = "gym-parser")]
#[command(about = "Parse and view log data for gym exercises", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse an exercise log file and display the parsed data
    Parse {
        #[arg(short, long)]
        file: String,
    },
    /// Show credits for this program
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            let input = fs::read_to_string(file)?;
            let parsed_data = parse_exercise_log(&input)?;

            println!("{:?}", parsed_data);
        }

        Commands::Credits => {
            println!("Simple parser for my personal gym data collected and written manually");
            println!("Created by Volodymyr Beimuk");
        }
    }

    Ok(())
}
