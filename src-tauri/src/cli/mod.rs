mod console;
mod headless;

use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::path::PathBuf;

pub use console::ensure_console;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GuiLaunchOptions {
    pub silent: bool,
    pub force_show: bool,
    pub open_note_id: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug)]
pub enum CliOutcome {
    Exit(i32),
    RunGui(GuiLaunchOptions),
}

#[derive(Parser)]
#[command(
    name = "floral-notepaper",
    version,
    about = "Floral Notepaper - lightweight local note app",
    disable_version_flag = true
)]
struct Cli {
    #[arg(short = 'V', long, global = true)]
    version: bool,

    #[command(subcommand)]
    command: Option<Command>,

    #[arg(long, global = true, help = "Start without showing the main window")]
    silent: bool,

    #[arg(long, global = true, help = "Show the main window on startup")]
    show: bool,

    #[arg(long, global = true, help = "Open a note by ID in the main editor")]
    open_note: Option<String>,

    #[arg(value_name = "FILE", help = "Open a markdown or text file")]
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Manage notes
    Notes {
        #[command(subcommand)]
        command: NotesCommand,
    },
    /// Manage categories
    Categories {
        #[command(subcommand)]
        command: CategoriesCommand,
    },
}

#[derive(Subcommand)]
enum NotesCommand {
    /// List all notes
    List {
        #[arg(long, help = "Output JSON")]
        json: bool,
    },
    /// Show a note by ID
    Get {
        id: String,
        #[arg(long, help = "Output JSON")]
        json: bool,
    },
    /// Create a new note
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long, help = "Output JSON")]
        json: bool,
    },
    /// Delete a note by ID
    Delete { id: String },
    /// Export a note to a markdown file
    Export {
        id: String,
        #[arg(long, short = 'o')]
        output: PathBuf,
    },
    /// Import a markdown file as a note
    Import {
        path: PathBuf,
        #[arg(long)]
        category: Option<String>,
        #[arg(long, help = "Output JSON")]
        json: bool,
    },
}

#[derive(Subcommand)]
enum CategoriesCommand {
    /// List all categories
    List {
        #[arg(long, help = "Output JSON")]
        json: bool,
    },
}

pub fn dispatch(args: impl IntoIterator<Item = impl Into<OsString>>) -> CliOutcome {
    let args: Vec<OsString> = args.into_iter().map(Into::into).collect();
    let argv: Vec<String> = args
        .iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();

    let parsed = match Cli::try_parse_from(argv) {
        Ok(parsed) => parsed,
        Err(error) => {
            ensure_console();
            if error.use_stderr() {
                eprintln!("{error}");
            } else {
                print!("{error}");
            }
            return CliOutcome::Exit(error.exit_code());
        }
    };

    if parsed.version {
        ensure_console();
        println!("floral-notepaper {}", env!("CARGO_PKG_VERSION"));
        return CliOutcome::Exit(0);
    }

    if let Some(command) = parsed.command {
        ensure_console();
        let exit_code = run_headless(command);
        return CliOutcome::Exit(exit_code);
    }

    CliOutcome::RunGui(GuiLaunchOptions {
        silent: parsed.silent,
        force_show: parsed.show,
        open_note_id: parsed.open_note,
        file_path: parsed
            .file
            .map(|path| path.to_string_lossy().into_owned())
            .or_else(|| extract_file_path_from_args(&args)),
    })
}

pub fn parse_second_instance_args(args: &[String]) -> GuiLaunchOptions {
    let argv = std::iter::once("floral-notepaper".to_string())
        .chain(args.iter().cloned())
        .collect::<Vec<_>>();

    match Cli::try_parse_from(argv) {
        Ok(parsed) => GuiLaunchOptions {
            silent: parsed.silent,
            force_show: parsed.show,
            open_note_id: parsed.open_note,
            file_path: parsed
                .file
                .map(|path| path.to_string_lossy().into_owned())
                .or_else(|| extract_file_path_from_strings(args)),
        },
        Err(_) => GuiLaunchOptions {
            file_path: extract_file_path_from_strings(args),
            ..GuiLaunchOptions::default()
        },
    }
}

fn run_headless(command: Command) -> i32 {
    let result = match command {
        Command::Notes { command } => match command {
            NotesCommand::List { json } => headless::run_notes_list(json),
            NotesCommand::Get { id, json } => headless::run_notes_get(&id, json),
            NotesCommand::Create {
                title,
                content,
                category,
                json,
            } => headless::run_notes_create(&title, content.as_deref(), category.as_deref(), json),
            NotesCommand::Delete { id } => headless::run_notes_delete(&id),
            NotesCommand::Export { id, output } => headless::run_notes_export(&id, &output),
            NotesCommand::Import {
                path,
                category,
                json,
            } => headless::run_notes_import(&path, category.as_deref(), json),
        },
        Command::Categories { command } => match command {
            CategoriesCommand::List { json } => headless::run_categories_list(json),
        },
    };

    match result {
        Ok(code) => code,
        Err(error) => {
            headless::print_cli_error(&error);
            1
        }
    }
}

fn extract_file_path_from_args(args: &[OsString]) -> Option<String> {
    let strings: Vec<String> = args
        .iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    extract_file_path_from_strings(&strings)
}

fn extract_file_path_from_strings(args: &[String]) -> Option<String> {
    crate::desktop::extract_file_arg(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_version_exits_zero() {
        match dispatch(["floral-notepaper", "--version"]) {
            CliOutcome::Exit(0) => {}
            other => panic!("expected version exit 0, got {other:?}"),
        }
    }

    #[test]
    fn dispatch_gui_mode_with_flags() {
        match dispatch([
            "floral-notepaper",
            "--silent",
            "--open-note",
            "note-1",
            "C:\\notes\\demo.md",
        ]) {
            CliOutcome::RunGui(options) => {
                assert!(options.silent);
                assert_eq!(options.open_note_id.as_deref(), Some("note-1"));
                assert_eq!(options.file_path.as_deref(), Some("C:\\notes\\demo.md"));
            }
            other => panic!("expected gui mode, got {other:?}"),
        }
    }

    #[test]
    fn dispatch_headless_notes_list() {
        match dispatch(["floral-notepaper", "notes", "list"]) {
            CliOutcome::Exit(code) => assert_eq!(code, 0),
            other => panic!("expected headless exit, got {other:?}"),
        }
    }

    #[test]
    fn parse_second_instance_args_reads_flags() {
        let options = parse_second_instance_args(&[
            "--silent".into(),
            "--open-note".into(),
            "abc".into(),
            "D:\\tmp\\note.md".into(),
        ]);
        assert!(options.silent);
        assert_eq!(options.open_note_id.as_deref(), Some("abc"));
        assert_eq!(options.file_path.as_deref(), Some("D:\\tmp\\note.md"));
    }
}
