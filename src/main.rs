use std::fs;
use clap::{Parser, Subcommand};
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::fs::File;
use std::path::Path;
use directories::ProjectDirs;

#[derive(Parser)]
#[command(version, about = "A simple CLI note app", name = "notes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{

    //Add a new note
    Add {
        text: Vec<String>
    },

    //List all notes
    List,

    //Remove all notes
    Clear
}

fn add_notes(file_directory: &Path, note: &str) -> std::io::Result<()>{
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_directory)?;
    writeln!(file, "{note}")?;
    Ok(())
}

fn clear_notes(file_directory: &Path) -> std::io::Result<()> {
    File::create(file_directory)?;
    Ok(())
}

fn list_notes(file_directory: &Path) {
    let contents = read_to_string(file_directory)
        .expect("Couldnt read note.txt");
    println!("{contents}")
}

fn main() {
    let cli = Cli::parse();
    let project_dirs = ProjectDirs::from("com", "JustinE", "note")
        .expect("Could not find a home directory");

    let data_dir = project_dirs.data_dir();
    fs::create_dir_all(data_dir)
        .expect("Could not create app directory");

    let directory = data_dir.join("notes.txt");

    match cli.command {
        Commands::Add { text } => {
            let note = text.join(" ");
            add_notes(&directory, &note).expect("Error with adding notes to notes.txt");
        }
        Commands::List => list_notes(&directory),
        Commands::Clear => clear_notes(&directory).expect("Error with clearing notes.txt")
    }
}