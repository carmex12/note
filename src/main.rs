use clap::{Parser, Subcommand};

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

    //Remove a note
    Remove {
        index: usize
    }
}

fn main() {

}