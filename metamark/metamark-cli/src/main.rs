use clap::{Parser, Subcommand};
use colored::*;
use metamark_core::{ast::Block, document::DocumentManager};
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "mmk")]
#[command(about = "MetaMark document processor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new MetaMark document
    New {
        /// Document title
        title: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Edit an existing document
    Edit {
        /// Document path
        path: PathBuf,
    },
    /// Export document to different formats
    Export {
        /// Input document path
        input: PathBuf,
        /// Output format (pdf, html, json, docx)
        #[arg(short, long)]
        format: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// List all documents in the working directory
    List,
    /// Show document information
    Info {
        /// Document path
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let manager = DocumentManager::new(std::env::current_dir()?);

    match cli.command {
        Commands::New { title, output } => {
            let doc = manager.create_document(&title)?;
            let path = output.unwrap_or_else(|| PathBuf::from(format!("{}.mmk", title)));
            manager.save_document(&doc, &path, false)?;
            info!("Created new document: {}", path.display());
        }
        Commands::Edit { path } => {
            let doc = manager.load_document(&path, None)?;
            // TODO: Implement interactive editing
            println!("Document content:\n{}", manager.export_mmk(&doc)?);
        }
        Commands::Export {
            input,
            format,
            output,
        } => {
            let doc = manager.load_document(&input, None)?;
            let output = output.unwrap_or_else(|| {
                let stem = input.file_stem().unwrap().to_str().unwrap();
                PathBuf::from(format!("{}.{}", stem, format))
            });

            match format.as_str() {
                "mmk" => {
                    let content = manager.export_mmk(&doc)?;
                    std::fs::write(&output, content)?;
                }
                // TODO: Implement other export formats
                _ => error!("Unsupported export format: {}", format),
            }

            info!("Exported document to: {}", output.display());
        }
        Commands::List => {
            let docs = manager.list_documents()?;
            if docs.is_empty() {
                println!("No documents found");
                return Ok(());
            }

            println!("Found {} documents:", docs.len());
            for doc in docs {
                println!(
                    "{}: {} ({})",
                    doc.path.display().to_string().blue(),
                    doc.metadata.title.green(),
                    doc.metadata.version
                );
            }
        }
        Commands::Info { path } => {
            let doc = manager.load_document(&path, None)?;
            println!("Document Information:");
            println!("  Title: {}", doc.metadata.title.green());
            println!("  Version: {}", doc.metadata.version);
            println!("  Created: {}", doc.metadata.created_at);
            println!("  Updated: {}", doc.metadata.updated_at);
            println!("  Authors: {}", doc.metadata.authors.join(", "));
            println!("  Tags: {}", doc.metadata.tags.join(", "));
            println!("\nContent Structure:");
            for (i, block) in doc.content.iter().enumerate() {
                match block {
                    Block::Heading { level, content, .. } => {
                        println!(
                            "  {}{} {}",
                            "  ".repeat(*level as usize),
                            format!("[{}]", i).blue(),
                            content
                        );
                    }
                    _ => println!("  {} {:?}", format!("[{}]", i).blue(), block),
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::tempdir;

    #[test]
    fn test_new_document() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test.mmk");

        let mut cmd = Command::cargo_bin("metamark-cli").unwrap();
        cmd.arg("new")
            .arg("Test Document")
            .arg("--output")
            .arg(&output_path)
            .assert()
            .success();

        assert!(output_path.exists());
    }
} 