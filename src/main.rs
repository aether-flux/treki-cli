mod cli;
mod commands;
mod utils;
mod auth;

use clap::Parser;
use cli::CliArgs;
use commands::handle_command;
use colored::*;

#[tokio::main]
async fn main() {
    const TREKI_ASC: &str = r#"
  ______          __   _ 
 /_  __/_______  / /__(_)
  / / / ___/ _ \/ //_/ / 
 / / / /  /  __/ ,< / /  
/_/ /_/   \___/_/|_/_/   
    "#;

    println!("{}\n\n", TREKI_ASC.truecolor(255, 165, 0));

    let args = CliArgs::try_parse().unwrap_or_else(|e| e.exit());  // Parses the arguments
    if let Err(e) = handle_command(args).await {
        eprintln!("❌ Error: {}", e);
    }
}
