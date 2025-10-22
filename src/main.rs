use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

// struct principale per la configurazione della CLI
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// sottocomando opzionale da eseguire
    #[command(subcommand)]
    command: Option<Commands>,
}

/// enum che definisce i sottocomandi disponibili
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
    /// Edit a note (vi default)
    Edit {
        #[arg(short, long)]
        title: String,
    },
}

// Stuttura dati per le note

/// struct che rappresenta la singola nota
/// derivata per la serializzazione
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

fn find_preferred_editor() -> String {
    // Prima controlla se vi è disponibile
    if Command::new("which")
        .arg("nvim")
        .output()
        .map_or(false, |output| output.status.success())
    {
        return "nvim".to_string();
    }

    // Se vi non è disponibile, controlla altre opzioni comuni
    let editors = vec!["vi", "nano", "vim", "gedit"];

    for editor in editors {
        if Command::new("which")
            .arg(editor)
            .output()
            .map_or(false, |output| output.status.success())
        {
            return editor.to_string();
        }
    }

    // Fallback a nano come default
    "nano".to_string()
}

fn open_editor_with_content(content: &str) -> io::Result<String> {
    // Crea un file temporaneo
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("note_edit_{}.txt", std::process::id()));

    // Scrivi il contenuto iniziale nel file temporaneo
    fs::write(&temp_file, content)?;

    // Trova l'editor preferito
    let editor = find_preferred_editor();

    // Apri l'editor
    let status = Command::new(&editor).arg(&temp_file).status()?;

    if !status.success() {
        fs::remove_file(&temp_file).ok(); // Ignora errori nella rimozione
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Editor {} exited with non-zero status", editor),
        ));
    }

    // Leggi il contenuto modificato
    let edited_content = fs::read_to_string(&temp_file)?;

    // Pulisci il file temporaneo
    fs::remove_file(&temp_file).ok(); // Ignora errori nella rimozione

    Ok(edited_content)
}

fn handle_new(title: &str) -> io::Result<()> {
    ensure_data_dir_exists()?;
    let path = get_note_path(title);

    if path.exists() {
        println!("Note with title '{}' already exists.", title);
        return Ok(());
    }

    let content = open_editor_with_content("")?;

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

    let mut entries: Vec<Note> = fs::read_dir(data_dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .map(|entry| {
            let path = entry.path();
            let file_content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&file_content).unwrap()
        })
        .collect();
    // Ordina le note per data di modifica (dalla più recente alla meno recente)
    entries.sort_by(|a, b| b.modification_date.cmp(&a.modification_date));

    loop {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let selected_title = run_list_app(&mut terminal, &mut entries)?;

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Some(title) = selected_title {
            if title == "__NEW__" {
                handle_new_from_list()?;
            } else {
                handle_external_edit(&title)?;
            }
        } else {
            break;
        }

        // Re-read the notes to reflect the changes
        entries = fs::read_dir(get_data_dir())?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
            .map(|entry| {
                let path = entry.path();
                let file_content = fs::read_to_string(&path).unwrap_or_default();
                serde_json::from_str(&file_content).unwrap()
            })
            .collect();
        // Riordina dopo le modifiche
        entries.sort_by(|a, b| b.modification_date.cmp(&a.modification_date));
    }

    Ok(())
}

fn handle_new_from_list() -> io::Result<()> {
    println!("Enter new note title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title)?;
    let title = title.trim();

    if title.is_empty() {
        println!("Title cannot be empty.");
        return Ok(());
    }

    handle_new(title)
}

fn handle_external_edit(title: &str) -> io::Result<()> {
    let path = get_note_path(title);
    if !path.exists() {
        println!("Note '{}' not found.", title);
        return Ok(());
    }

    let file_content = fs::read_to_string(&path)?;
    let mut note: Note = serde_json::from_str(&file_content)?;

    let edited_content = open_editor_with_content(&note.content)?;

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

fn run_list_app<B: Backend>(
    terminal: &mut Terminal<B>,
    notes: &mut Vec<Note>,
) -> io::Result<Option<String>> {
    let mut table_state = TableState::default();
    if !notes.is_empty() {
        table_state.select(Some(0));
    }

    loop {
        terminal.draw(|f| ui_list(f, &mut table_state, notes))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => return Ok(None),
                KeyCode::Char('n') if key.modifiers == KeyModifiers::CONTROL => {
                    // This will exit the list view and prompt for a new note title
                    return Ok(Some("__NEW__".to_string()));
                }
                KeyCode::Down => {
                    if !notes.is_empty() {
                        let i = match table_state.selected() {
                            Some(i) => {
                                if i >= notes.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        table_state.select(Some(i));
                    }
                }
                KeyCode::Up => {
                    if !notes.is_empty() {
                        let i = match table_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    notes.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        table_state.select(Some(i));
                    }
                }
                KeyCode::Enter => {
                    if let Some(selected) = table_state.selected() {
                        if let Some(note) = notes.get(selected) {
                            return Ok(Some(note.title.clone()));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn ui_list(f: &mut Frame, table_state: &mut TableState, notes: &[Note]) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(main_chunks[0]);

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Title", "Modified"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = notes.iter().map(|item| {
        let modified_date = item
            .modification_date
            .replace("T", " ")
            .split(".")
            .next()
            .unwrap_or("")
            .to_string();
        let cells = vec![
            Cell::from(item.title.clone()),
            Cell::from(modified_date.to_string()),
        ];
        Row::new(cells).height(1)
    });
    let t = Table::new(
        rows,
        &[Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Notes")
            .border_type(BorderType::Rounded),
    )
    .highlight_style(selected_style)
    .highlight_symbol(">> ");
    f.render_stateful_widget(t, top_chunks[0], table_state);

    let selected_note_content = if let Some(selected) = table_state.selected() {
        notes.get(selected).map_or("", |n| &n.content)
    } else {
        ""
    };

    let content_p = Paragraph::new(selected_note_content)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Preview")
                .border_type(BorderType::Rounded),
        );
    f.render_widget(content_p, top_chunks[1]);

    let footer = Paragraph::new("Ctrl-Q: Quit | Enter: Edit | Ctrl-N: New | ↑/↓: Navigate")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    f.render_widget(footer, main_chunks[1]);
}

fn handle_show(title: &str) -> io::Result<()> {
    let path = get_note_path(title);
    if !path.exists() {
        println!("Note '{}' not found.", title);
        return Ok(());
    }

    let file_content = fs::read_to_string(&path)?;
    let note: Note = serde_json::from_str(&file_content)?;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_show_app(&mut terminal, &note);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_show_app<B: Backend>(terminal: &mut Terminal<B>, note: &Note) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui_show(f, note))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui_show(f: &mut Frame, note: &Note) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Length(3), // Metadata
                Constraint::Min(1),    // Content
            ]
            .as_ref(),
        )
        .split(size);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Note ")
        .border_type(BorderType::Rounded);
    f.render_widget(block, size);

    let title = Paragraph::new(note.title.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Title"));
    f.render_widget(title, chunks[0]);

    let metadata_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let creation_date = Paragraph::new(note.creation_date.as_str())
        .block(Block::default().borders(Borders::ALL).title("Created"));
    f.render_widget(creation_date, metadata_layout[0]);

    let modification_date = Paragraph::new(note.modification_date.as_str())
        .block(Block::default().borders(Borders::ALL).title("Modified"));
    f.render_widget(modification_date, metadata_layout[1]);

    let content = Paragraph::new(note.content.as_str())
        .wrap(Wrap { trim: true })
        .block(Block::default().borders(Borders::ALL).title("Content"));
    f.render_widget(content, chunks[2]);
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

    let edited_content = open_editor_with_content(&note.content)?;

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
        Some(Commands::New { title }) => handle_new(title),
        Some(Commands::List) => handle_list(),
        Some(Commands::Show { title }) => handle_show(title),
        Some(Commands::Delete { title }) => handle_delete(title),
        Some(Commands::Edit { title }) => handle_edit(title),
        None => handle_list(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
