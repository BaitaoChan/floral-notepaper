use crate::services::notes::{default_store, AppError, Note, NoteMetadata, SaveNoteRequest};
use std::path::Path;

pub fn run_notes_list(json: bool) -> Result<i32, AppError> {
    let notes = default_store()?.list_notes()?;
    if json {
        println!("{}", serde_json::to_string_pretty(&notes)?);
    } else {
        print_notes_table(&notes);
    }
    Ok(0)
}

pub fn run_notes_get(id: &str, json: bool) -> Result<i32, AppError> {
    let note = default_store()?.read_note(id)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&note)?);
    } else {
        print_note_detail(&note);
    }
    Ok(0)
}

pub fn run_notes_create(
    title: &str,
    content: Option<&str>,
    category: Option<&str>,
    json: bool,
) -> Result<i32, AppError> {
    let note = default_store()?.create_note(SaveNoteRequest {
        title: title.to_string(),
        content: content.unwrap_or("").to_string(),
        category: category.unwrap_or("").to_string(),
    })?;
    if json {
        println!("{}", serde_json::to_string_pretty(&note)?);
    } else {
        println!("Created note {} ({})", note.title, note.id);
    }
    Ok(0)
}

pub fn run_notes_delete(id: &str) -> Result<i32, AppError> {
    default_store()?.delete_note(id)?;
    println!("Deleted note {id}");
    Ok(0)
}

pub fn run_notes_export(id: &str, output: &Path) -> Result<i32, AppError> {
    default_store()?.export_markdown_file(id, output)?;
    println!("Exported note {id} to {}", output.display());
    Ok(0)
}

pub fn run_notes_import(path: &Path, category: Option<&str>, json: bool) -> Result<i32, AppError> {
    let note = default_store()?.import_markdown_file(path, category.unwrap_or(""))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&note)?);
    } else {
        println!(
            "Imported {} as note {} ({})",
            path.display(),
            note.title,
            note.id
        );
    }
    Ok(0)
}

pub fn run_categories_list(json: bool) -> Result<i32, AppError> {
    let categories = default_store()?.list_categories()?;
    if json {
        println!("{}", serde_json::to_string_pretty(&categories)?);
    } else if categories.is_empty() {
        println!("No categories found.");
    } else {
        for name in categories {
            println!("{name}");
        }
    }
    Ok(0)
}

fn print_notes_table(notes: &[NoteMetadata]) {
    if notes.is_empty() {
        println!("No notes found.");
        return;
    }

    println!("{:<36}  {:<24}  {:<16}  UPDATED", "ID", "TITLE", "CATEGORY");
    for note in notes {
        println!(
            "{:<36}  {:<24}  {:<16}  {}",
            note.id,
            truncate(&note.title, 24),
            truncate(&note.category, 16),
            note.updated_at.format("%Y-%m-%d %H:%M")
        );
    }
}

fn print_note_detail(note: &Note) {
    println!("ID:       {}", note.id);
    println!("Title:    {}", note.title);
    println!("Category: {}", note.category);
    println!(
        "Updated:  {}",
        note.updated_at.format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!("Words:    {}", note.word_count);
    println!("---");
    println!("{}", note.content);
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let trimmed: String = value.chars().take(max_chars.saturating_sub(1)).collect();
    format!("{trimmed}…")
}

pub fn print_cli_error(error: &AppError) {
    eprintln!("error: {error}");
}
