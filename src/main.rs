use anyhow::{anyhow, Result};
use std::fs;
use structopt::StructOpt;

mod cli;
mod task;

use cli::{Action, CommandLineArgs};
use task::{add_task, delete_tasks, list_tasks, update_task_status, Status, Task};

fn find_default_journal_file() -> Option<std::path::PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".config/Rust-Kanban/rust-kanban.json");
        path
    })
}

fn main() -> Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or_else(|| anyhow!("Failed to find journal file."))?;

    if let Some(parent) = journal_file.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    match action {
        Action::Add { task } => {
            add_task(&journal_file, Task::new(task))?;
            println!("Task added successfully.");
        }
        Action::List => list_tasks(&journal_file)?,
        Action::Doing { position } => {
            update_task_status(&journal_file, &position, Status::Doing)?;
            println!("Task(s) marked as doing.");
        }
        Action::Done { position } => {
            update_task_status(&journal_file, &position, Status::Done)?;
            println!("Task(s) marked as done.");
        }
        Action::Delete { position } => {
            delete_tasks(&journal_file, &position)?;
            println!("Task(s) deleted.");
        }
    }
    Ok(())
}