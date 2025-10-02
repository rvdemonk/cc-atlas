use anyhow::Result;
use clap::{Parser, Subcommand};

mod models;
mod server;
mod services;
mod utils;

use services::{analyzer, chat_exporter};
use models::ExportOptions;
use std::path::PathBuf;

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

    /// List all chat transcripts for a project
    ListChats {
        #[arg(short, long, default_value = ".")]
        project: PathBuf,
    },

    /// Export a chat transcript to markdown
    ExportChat {
        #[arg(help = "Session ID or index from list-chats (1, 2, 3...)")]
        identifier: String,

        #[arg(short, long, default_value = ".")]
        project: PathBuf,
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
        Some(Commands::ListChats { project }) => {
            handle_list_chats(&project)?;
        }
        Some(Commands::ExportChat { identifier, project }) => {
            handle_export_chat(&identifier, &project)?;
        }
        None => {
            println!("Starting cc-atlas server on default port 3999");
            server::run(3999, ".".to_string()).await?;
        }
    }
    
    Ok(())
}

fn handle_list_chats(project: &PathBuf) -> Result<()> {
    let chats = chat_exporter::find_project_chats(project)?;

    if chats.is_empty() {
        println!("No chats found for this project.");
        return Ok(());
    }

    println!("Found {} chat{}:\n", chats.len(), if chats.len() == 1 { "" } else { "s" });

    for (i, chat) in chats.iter().enumerate() {
        println!("{}. {}", i + 1, chat.title);
        println!("   Session ID: {}", chat.session_id);
        println!("   Size: {} bytes\n", chat.file_size);
    }

    Ok(())
}

fn handle_export_chat(identifier: &str, project: &PathBuf) -> Result<()> {
    // Check if identifier is a number (index) or session ID
    let session_id = if let Ok(index) = identifier.parse::<usize>() {
        // It's an index - look up the session ID
        let chats = chat_exporter::find_project_chats(project)?;

        if index == 0 || index > chats.len() {
            anyhow::bail!("Index out of range. Use 1-{} or a session ID.", chats.len());
        }

        chats[index - 1].session_id.clone()
    } else {
        // It's a session ID
        identifier.to_string()
    };

    let result = chat_exporter::export_chat(
        &session_id,
        project,
        &ExportOptions::default()
    )?;

    println!("✅ Exported {} message{} to:",
        result.message_count,
        if result.message_count == 1 { "" } else { "s" }
    );
    println!("   {}", result.output_path.display());

    Ok(())
}