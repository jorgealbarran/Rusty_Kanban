// Import necessary libraries for handling file paths and command-line arguments
use std::path::PathBuf;
use structopt::StructOpt;

// Define the possible actions the user can perform
#[derive(Debug, StructOpt)]
pub enum Action {
    // Action to add a new task
    #[structopt(about = "Add a new task to the to-do list")]
    Add {
        #[structopt(help = "Task description")]
        task: String, // The description of the task to be added
    },
    // Action to mark a task as "doing"
    #[structopt(about = "Mark task(s) as doing")]
    Doing {
        #[structopt(help = "Position of the task(s) to mark as doing")]
        position: Vec<usize>, // A vector of task positions to be marked as doing
    },
    // Action to mark a task as "done"
    #[structopt(about = "Mark task(s) as done")]
    Done {
        #[structopt(help = "Position of the task(s) to mark as done")]
        position: Vec<usize>, // A vector of task positions to be marked as done
    },
    // Action to delete a task
    #[structopt(about = "Delete task(s) from the to-do list")]
    Delete {
        #[structopt(help = "Position of the task(s) to delete")]
        position: Vec<usize>, // A vector of task positions to be deleted
    },
    // Action to list all tasks
    #[structopt(about = "List all tasks")]
    List,
}

// Define the structure for command-line arguments
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Kanban",
    about = "A command line Kanban app written in Rust",
    author = "akash2061 modified by Jorge Albarran"
)]
pub struct CommandLineArgs {
    // The specific action to be performed
    #[structopt(subcommand)]
    pub action: Action,

    // Optional path to the journal file
    #[structopt(
        parse(from_os_str),
        short = "f",
        long = "journal-file",
        help = "Path to the journal file"
    )]
    pub journal_file: Option<PathBuf>,
}