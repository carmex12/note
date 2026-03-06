use clap::{Parser, Subcommand};
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::fs::File;
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

fn add_notes(note: &str) -> std::io::Result<()>{
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")?;
    writeln!(file, "{note}")?;
    Ok(())
}

fn clear_notes() -> std::io::Result<()> {
    File::create("notes.txt")?;
    Ok(())
}

fn list_notes() {
    let contents = read_to_string("notes.txt")
        .expect("Couldnt read note.txt");
    println!("{contents}")
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text } => {
            let note = text.join(" ");
            add_notes(&note).expect("Error with adding notes to notes.txt");
        }
        Commands::List => list_notes(),
        Commands::Clear => clear_notes().expect("Error with clearing notes.txt")
    }
}