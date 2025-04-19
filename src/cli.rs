use clap::{Parser, Subcommand};

// Initialise the command
#[derive(Parser)]
#[command(name="treki", about="Lightweight API tester in your terminal")]
pub struct CliArgs {
    #[arg(short, long, global=true, help="Display additional information, like headers and status codes")]
    pub verbose: bool,  // Global flag: verbose mode

    #[command(subcommand)]
    pub command: Commands,  // Other commands
}

// Other commands
#[derive(Subcommand)]
pub enum Commands {
    // GET request
    Get {
        url: String,

        #[arg(short='H', long, help="Custom headers: 'key1:value1, key2:value2'")]
        headers: Option<String>,
    },
    
    // POST request
    Post {
        url: String,

        #[arg(short, long, help="Request body as raw string (e.g. JSON)")]
        body: Option<String>,

        #[arg(short='H', long, help="Custom headers: 'key1:value1, key2:value2'")]
        headers: Option<String>,
    },

    // PUT request
    Put {
        url: String,

        #[arg(short, long, help="Request body as raw string (e.g. JSON)")]
        body: Option<String>,

        #[arg(short='H', long, help="Custom headers: 'key1:value1, key2:value2'")]
        headers: Option<String>,
    },

    // PATCH request
    Patch {
        url: String,

        #[arg(short, long, help="Request body as raw string (e.g. JSON)")]
        body: Option<String>,

        #[arg(short='H', long, help="Custom headers: 'key1:value1, key2:value2'")]
        headers: Option<String>,
    },

    // DELETE request
    Delete {
        url: String,

        #[arg(short, long, help="Request body as raw string (e.g. JSON)")]
        body: Option<String>,

        #[arg(short='H', long, help="Custom headers: 'key1:value1, key2:value2'")]
        headers: Option<String>,
    },

    // Auth commands
    Login,
    Whoami,

    // Run saved request by request-ID
    Run {
        id: usize,
    }
}
