// Import necessary libraries and modules
use anyhow::{anyhow, Result}; // For error handling
use std::fs; // For file system operations
use structopt::StructOpt; // For parsing command-line arguments

// Declare modules for command-line interface and task management
mod cli;
mod task;

// Use specific items from the declared modules
use cli::{Action, CommandLineArgs};
use task::{add_task, doing_task, done_task, list_tasks, Task};

// Use the delete_task function from the task module
use crate::task::delete_task;

// Function to find the default journal file path
fn find_default_journal_file() -> Option<std::path::PathBuf> {
    // Get the home directory
    home::home_dir().map(|mut path| {
        // Append the default path for the journal file
        path.push(".config/Rust-Kanban/rust-kanban.json");
        path
    })
}

// Main function where the program execution starts
fn main() -> Result<()> {
    // Parse command-line arguments
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Determine the journal file path to be used
    let journal_file = journal_file
        .or_else(find_default_journal_file) // Use default if not provided
        .ok_or(anyhow!("Failed to find journal file."))?; // Error if no path is found

    // Create the directory for the journal file if it doesn't exist
    if let Some(parent) = journal_file.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    // Match the action specified by the user
    match action {
        // If the action is to add a task
        Action::Add { task } => {
            add_task(&journal_file, Task::new(task))?; // Add the new task
            println!("Task added successfully.");
        }
        // If the action is to list all tasks
        Action::List => list_tasks(&journal_file)?,
        // If the action is to mark a task as "doing"
        Action::Doing { mut position } => {
            position.sort_by(|a, b| b.cmp(a)); // Sort positions in descending order
            let cloned_position = position.clone();
            doing_task(&journal_file, cloned_position)?; // Mark tasks as "doing"
            for pos in position {
                println!("Task at position {} marked as doing.", pos);
            }
        }
        // If the action is to mark a task as "done"
        Action::Done { mut position } => {
            position.sort_by(|a, b| b.cmp(a)); // Sort positions in descending order
            let cloned_position = position.clone();
            done_task(&journal_file, cloned_position)?; // Mark tasks as "done"
            for pos in position {
                println!("Task at position {} marked as done.", pos);
            }
        }
        // If the action is to delete a task
        Action::Delete { mut position } => {
            position.sort_by(|a, b| b.cmp(a)); // Sort positions in descending order
            for pos in position {
                delete_task(journal_file.clone(), pos)?; // Delete the task
                println!("Task at position {} deleted.", pos);
            }
        }
    }
    // Return Ok if everything executed successfully
    Ok(())
}
