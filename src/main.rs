use anyhow::Result;
use clap::{Parser, Subcommand};

mod analyzer;
mod server;
mod models;

#[derive(Parser)]
#[command(name = "cc-atlas")]
#[command(about = "Context management for Claude Code CLAUDE.md files")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the web server
    Serve {
        #[arg(long, default_value_t = 3999)]
        port: u16,
        
        #[arg(short, long, default_value = ".")]
        project: String,
    },
    
    /// Analyze project without starting server
    Analyze {
        #[arg(default_value = ".")]
        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Serve { port, project }) => {
            println!("Starting cc-atlas server on port {} for project: {}", port, project);
            server::run(port, project).await?;
        }
        Some(Commands::Analyze { path }) => {
            println!("Analyzing project at: {}", path);
            analyzer::analyze_project(&path)?;
        }
        None => {
            println!("Starting cc-atlas server on default port 3999");
            server::run(3999, ".".to_string()).await?;
        }
    }
    
    Ok(())
}