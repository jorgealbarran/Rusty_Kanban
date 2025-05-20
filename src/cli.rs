use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    #[structopt(about = "Add a new task to the to-do list")]
    Add {
        #[structopt(help = "Task description")]
        task: String,
    },
    #[structopt(about = "Mark task(s) as doing")]
    Doing {
        #[structopt(help = "Position of the task(s) to mark as doing")]
        position: Vec<usize>,
    },
    #[structopt(about = "Mark task(s) as done")]
    Done {
        #[structopt(help = "Position of the task(s) to mark as done")]
        position: Vec<usize>,
    },
    #[structopt(about = "Delete task(s) from the to-do list")]
    Delete {
        #[structopt(help = "Position of the task(s) to delete")]
        position: Vec<usize>,
    },
    #[structopt(about = "List all tasks")]
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Kanban",
    about = "A command line Kanban app written in Rust",
    author = "akash2061 modified"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(
        parse(from_os_str),
        short = "f",
        long = "journal-file",
        help = "Path to the journal file"
    )]
    pub journal_file: Option<PathBuf>,
}
