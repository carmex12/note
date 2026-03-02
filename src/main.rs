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
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text } => {
            let note = text.join(" ");
            //Store message in .notes.json file
            todo!()
        }
        Commands::List => {
            //Print out all currents notes in .notes.json
            todo!()
        }
        Commands::Remove { index } => {
            //Remove the note in the index given with the given index
            todo!()
        }
    }
}