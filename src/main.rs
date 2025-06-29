use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new note
    New {
        #[arg(short, long)]
        title: String,
    },
    /// List all notes
    List,
    /// Show a specific note
    Show {
        #[arg(short, long)]
        title: String,
    },
    /// Delete a note
    Delete {
        #[arg(short, long)]
        title: String,
    },
    /// Edit a note
    Edit {
        #[arg(short, long)]
        title: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    title: String,
    content: String,
    creation_date: String,
    modification_date: String,
}

fn get_data_dir() -> PathBuf {
    let mut path = std::env::current_dir().expect("Failed to get current directory");
    path.push("data");
    path
}

fn get_note_path(title: &str) -> PathBuf {
    let mut path = get_data_dir();
    let filename = format!("{}.json", title.replace(" ", "_").to_lowercase());
    path.push(filename);
    path
}

fn ensure_data_dir_exists() -> io::Result<()> {
    let path = get_data_dir();
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn handle_new(title: &str) -> io::Result<()> {
    ensure_data_dir_exists()?;
    let path = get_note_path(title);

    if path.exists() {
        println!("Note with title '{}' already exists.", title);
        return Ok(());
    }

    let content = edit::edit("")?;

    let now = chrono::Utc::now().to_rfc3339();

    let note = Note {
        title: title.to_string(),
        content,
        creation_date: now.clone(),
        modification_date: now,
    };

    let json = serde_json::to_string_pretty(&note)?;
    fs::write(&path, json)?;

    println!("Note '{}' created successfully.", title);
    Ok(())
}

fn handle_list() -> io::Result<()> {
    let data_dir = get_data_dir();
    if !data_dir.exists() {
        println!("No notes found. The 'data' directory doesn't exist.");
        return Ok(());
    }

    let entries = fs::read_dir(data_dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"));

    println!("{:<20} {:<30}", "TITLE", "CREATION DATE");
    println!("{}", "-".repeat(50));

    for entry in entries {
        let path = entry.path();
        let file_content = fs::read_to_string(&path)?;
        let note: Note = serde_json::from_str(&file_content)?;
        println!("{:<20} {:<30}", note.title, note.creation_date);
    }

    Ok(())
}

fn handle_show(title: &str) -> io::Result<()> {
    let path = get_note_path(title);
    if !path.exists() {
        println!("Note '{}' not found.", title);
        return Ok(());
    }

    let file_content = fs::read_to_string(&path)?;
    let note: Note = serde_json::from_str(&file_content)?;

    println!("Title: {}", note.title);
    println!("Created: {}", note.creation_date);
    println!("Modified: {}", note.modification_date);
    println!("{}", "-".repeat(40));
    println!("{}", note.content);

    Ok(())
}

fn handle_delete(title: &str) -> io::Result<()> {
    let path = get_note_path(title);
    if !path.exists() {
        println!("Note '{}' not found.", title);
        return Ok(());
    }

    fs::remove_file(&path)?;
    println!("Note '{}' deleted successfully.", title);
    Ok(())
}

fn handle_edit(title: &str) -> io::Result<()> {
    let path = get_note_path(title);
    if !path.exists() {
        println!("Note '{}' not found.", title);
        return Ok(());
    }

    let file_content = fs::read_to_string(&path)?;
    let mut note: Note = serde_json::from_str(&file_content)?;

    let edited_content = edit::edit(&note.content)?;

    if edited_content != note.content {
        note.content = edited_content;
        note.modification_date = chrono::Utc::now().to_rfc3339();
        let json = serde_json::to_string_pretty(&note)?;
        fs::write(&path, json)?;
        println!("Note '{}' edited successfully.", title);
    } else {
        println!("No changes made to the note.");
    }

    Ok(())
}


fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::New { title } => handle_new(title),
        Commands::List => handle_list(),
        Commands::Show { title } => handle_show(title),
        Commands::Delete { title } => handle_delete(title),
        Commands::Edit { title } => handle_edit(title),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
