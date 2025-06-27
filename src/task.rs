// Import necessary libraries for date/time, colored output, serialization, etc.
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt; // For custom formatting
use std::fs::{File, OpenOptions}; // For file operations
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom}; // For I/O operations and error handling
use std::path::PathBuf; // For handling file paths

// Define the structure for a Task
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String, // The description of the task

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>, // The timestamp when the task was created

    #[serde(default)]
    pub doing: bool, // Flag to indicate if the task is in progress
    pub done: bool,  // Flag to indicate if the task is completed
}

// Implementation block for the Task struct
impl Task {
    // Function to create a new task
    pub fn new(text: String) -> Self {
        Task {
            text,
            created_at: Utc::now(), // Set the creation time to the current time
            doing: false,            // Initially, the task is not in progress
            done: false,             // Initially, the task is not completed
        }
    }
}

// Function to add a new task to the journal file
pub fn add_task(journal_path: &PathBuf, task: Task) -> Result<()> {
    // Open the journal file with read, write, and create permissions
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Collect all existing tasks from the file
    let mut tasks = collect_tasks(&file)?;

    // Add the new task to the list
    tasks.push(task);
    // Write the updated list of tasks back to the file
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

// Function to mark one or more tasks as "doing"
pub fn doing_task(journal_path: &PathBuf, task_positions: Vec<usize>) -> Result<()> {
    // Open the journal file with read and write permissions
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Collect all existing tasks from the file
    let mut tasks = collect_tasks(&file)?;

    // Validate the provided task positions
    for &pos in &task_positions {
        if pos == 0 || pos > tasks.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
        }
    }

    // Mark the specified tasks as "doing"
    for &pos in &task_positions {
        tasks[pos - 1].doing = true;
        tasks[pos - 1].done = false;
    }

    // Partition the tasks into "done" and "incomplete"
    let (done_tasks, incomplete_tasks): (Vec<Task>, Vec<Task>) =
        tasks.into_iter().partition(|task| task.done);

    // Partition the incomplete tasks into "doing" and "to-do"
    let (doing_tasks, todo_tasks): (Vec<Task>, Vec<Task>) =
        incomplete_tasks.into_iter().partition(|task| task.doing);

    // Create a sorted list of tasks: to-do, then doing, then done
    let mut sorted_tasks: Vec<Task> = todo_tasks;
    sorted_tasks.extend(doing_tasks);
    sorted_tasks.extend(done_tasks);

    // Open the journal file with write and truncate permissions to overwrite it
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(journal_path)?;

    // Write the sorted list of tasks back to the file
    serde_json::to_writer(file, &sorted_tasks)?;

    Ok(())
}

// Function to mark one or more tasks as "done"
pub fn done_task(journal_path: &PathBuf, task_positions: Vec<usize>) -> Result<()> {
    // Open the journal file with read and write permissions
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Collect all existing tasks from the file
    let mut tasks = collect_tasks(&file)?;

    // Validate the provided task positions
    for &pos in &task_positions {
        if pos == 0 || pos > tasks.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
        }
    }

    // Mark the specified tasks as "done"
    for &pos in &task_positions {
        tasks[pos - 1].doing = false;
        tasks[pos - 1].done = true;
    }

    // Partition the tasks into "done" and "incomplete"
    let (done_tasks, incomplete_tasks): (Vec<Task>, Vec<Task>) =
        tasks.into_iter().partition(|task| task.done);

    // Partition the incomplete tasks into "doing" and "to-do"
    let (doing_tasks, todo_tasks): (Vec<Task>, Vec<Task>) =
        incomplete_tasks.into_iter().partition(|task| task.doing);

    // Create a sorted list of tasks: to-do, then doing, then done
    let mut sorted_tasks: Vec<Task> = todo_tasks;
    sorted_tasks.extend(doing_tasks);
    sorted_tasks.extend(done_tasks);

    // Open the journal file with write and truncate permissions to overwrite it
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(journal_path)?;

    // Write the sorted list of tasks back to the file
    serde_json::to_writer(file, &sorted_tasks)?;

    Ok(())
}

// Function to delete a task from the journal file
pub fn delete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    // Open the journal file with read and write permissions
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Collect all existing tasks from the file
    let mut tasks = collect_tasks(&file)?;

    // Validate the provided task position
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    // Remove the specified task
    tasks.remove(task_position - 1);

    // Truncate the file to clear its contents
    file.set_len(0)?;
    // Write the updated list of tasks back to the file
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

// Function to list all tasks in the journal file
pub fn list_tasks(journal_path: &PathBuf) -> Result<()> {
    // Open the journal file with read permissions
    let file = OpenOptions::new().read(true).open(journal_path)?;
    // Collect all existing tasks from the file
    let tasks = collect_tasks(&file)?;

    // Check if the task list is empty
    if tasks.is_empty() {
        println!("{}", "Task list is empty!".yellow());
    } else {
        let mut order: u32 = 1;
        println!("====================================");
        println!("{}", "To Do:".blue());
        // Iterate over and print tasks that are not "doing" and not "done"
        for (_index, task) in tasks.iter().enumerate().filter(|(_, task)| !task.doing && !task.done ) {
            println!("{}: {}", order, task);
            order += 1;
        }
        println!("-------------------------------");
        println!("{}", "Doing:".green());
        // Iterate over and print tasks that are "doing"
        if tasks.iter().any(|task| task.doing) {
            for (index, task) in tasks.iter().enumerate().filter(|(_, task)| task.doing && !task.done) {
                println!("{}: {}", index + 1, task);
            }
        }
        println!("-------------------------------");
        println!("{}", "Done:".red());
        // Iterate over and print tasks that are "done"
        if tasks.iter().any(|task| task.done) {
            for (index, task) in tasks.iter().enumerate().filter(|(_, task)| task.done && !task.doing) {
                println!("{}: {}", index + 1, task);
            }
        }
        println!("====================================");
    }

    Ok(())
}

// Function to collect all tasks from the journal file
fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    // Seek to the beginning of the file
    file.seek(SeekFrom::Start(0))?;
    // Deserialize the JSON content of the file into a vector of tasks
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(), // If the file is empty, return an empty vector
        Err(e) => Err(e)?, // If there is any other error, return it
    };
    // Seek back to the beginning of the file
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

// Implementation of the Display trait for the Task struct
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the creation timestamp
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        // Write the formatted task to the formatter
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}
