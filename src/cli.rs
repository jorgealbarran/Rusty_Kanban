// Import necessary libraries for handling file paths and command-line arguments
use std::path::PathBuf;
use structopt::StructOpt;

// Define the possible actions the user can perform
#[derive(Debug, StructOpt)]
pub enum Action {
    // Action to add a new task
    #[structopt(about = "Add a new task to the to-do list")]
    Add {
        #[structopt(help = "The description of the task to add")]
        task: String,
    },
    // Action to mark a task as "doing"
    #[structopt(about = "Mark one or more tasks as 'doing'")]
    Doing {
        #[structopt(help = "The position(s) of the task(s) to mark as doing")]
        position: Vec<usize>,
    },
    // Action to mark a task as "done"
    #[structopt(about = "Mark one or more tasks as 'done'")]
    Done {
        #[structopt(help = "The position(s) of the task(s) to mark as done")]
        position: Vec<usize>,
    },
    // Action to delete a task
    #[structopt(about = "Delete one or more tasks from the to-do list")]
    Delete {
        #[structopt(help = "The position(s) of the task(s) to delete")]
        position: Vec<usize>,
    },
    // Action to list all tasks
    #[structopt(about = "List all tasks in the Kanban board")]
    List,
}

// Define the structure for command-line arguments
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Kanban",
    about = "A simple command-line Kanban board application written in Rust.",
    author = "Jorge Albarran"
)]
pub struct CommandLineArgs {
    // The specific action to be performed
    #[structopt(subcommand)]
    pub action: Action,

    // Optional path to the journal file
    #[structopt(
        parse(from_os_str),
        short = "f",
        long = "file",
        help = "Path to the journal file. Defaults to ~/.config/Rust-Kanban/rust-kanban.json"
    )]
    pub journal_file: Option<PathBuf>,
}