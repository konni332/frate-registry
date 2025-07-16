use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "frate-registry-gen")]
#[command(about = "Generate frate-compatible registry JSON from GitHub releases", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan GitHub repo and generate a registry file
    Generate {
        /// Name of the tool (e.g., just, ripgrep)
        #[arg(long)]
        name: String,

        /// GitHub repo (e.g., casey/just, no "https://github.com/" !)
        #[arg(long)]
        repo: String,

        /// Output file (defaults to tools/<name>.json)
        #[arg(long)]
        out: Option<String>,

        /// Max number of releases to be registered
        #[arg(long)]
        max: Option<usize>,
    },
    FromList {
        /// File to get the names and repos from
        file: String,
    }
}
