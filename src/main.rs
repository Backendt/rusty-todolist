mod note_model;
mod note_service;
mod note_repository;

use clap::{Parser, Subcommand};
use crate::note_service::*;

/// List, add, delete or read notes
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The command to use on note(s)
    #[command(subcommand)]
    command: Commands,
    /// If title should be interpreted as a regular expression
    #[arg(short, long)]
    regex: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List notes
    #[clap(visible_alias = "ls")]
    List,
    /// Add note
    Add {
        title: String,
        content: String,
    },
    /// Delete note(s)
    #[clap(visible_alias = "rm")]
    Delete {
        title: String,
    },
    /// Read note(s)
    #[clap(visible_alias = "see")]
    Read {
        title: String,
    },
}

fn main() {
    let command_args = Cli::parse();
    match &command_args.command {
        Commands::List => list_notes(),
        Commands::Add{title, content} => add_note(title, content),
        Commands::Delete{title} => delete_note(title, command_args.regex),
        Commands::Read{title} => read_note(title, command_args.regex),
    };
}
