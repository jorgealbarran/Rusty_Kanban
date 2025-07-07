// Import necessary libraries for date/time, colored output, serialization, etc.
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

// Define the structure for a Task
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

impl Default for Status {
    fn default() -> Self {
        Status::Todo
    }
}

impl Task {
    pub fn new(text: String) -> Self {
        Task {
            text,
            created_at: Utc::now(),
            status: Status::Todo,
        }
    }
}

fn read_tasks(file: &mut File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(&*file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => return Err(e.into()),
    };
    Ok(tasks)
}

fn write_tasks(file: &mut File, tasks: &[Task]) -> Result<()> {
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer(file, tasks)?;
    Ok(())
}

pub fn add_task(journal_path: &PathBuf, task: Task) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    let mut tasks = read_tasks(&mut file)?;
    tasks.push(task);
    write_tasks(&mut file, &tasks)
}

pub fn update_task_status(journal_path: &PathBuf, task_positions: &[usize], status: Status) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    let mut tasks = read_tasks(&mut file)?;

    for &pos in task_positions {
        if pos == 0 || pos > tasks.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
        }
        tasks[pos - 1].status = status.clone();
    }

    tasks.sort_by_key(|task| match task.status {
        Status::Todo => 0,
        Status::Doing => 1,
        Status::Done => 2,
    });

    write_tasks(&mut file, &tasks)
}

pub fn delete_tasks(journal_path: &PathBuf, task_positions: &[usize]) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    let mut tasks = read_tasks(&mut file)?;

    let mut positions = task_positions.to_vec();
    positions.sort_unstable();
    positions.reverse();

    for &pos in &positions {
        if pos == 0 || pos > tasks.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
        }
        tasks.remove(pos - 1);
    }

    write_tasks(&mut file, &tasks)
}

pub fn list_tasks(journal_path: &PathBuf) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = read_tasks(&mut file)?;

    if tasks.is_empty() {
        println!("{}", "Task list is empty!".yellow());
    } else {
        println!("====================================");
        print_tasks_by_status(&tasks, Status::Todo, "To Do:".blue());
        println!("-------------------------------");
        print_tasks_by_status(&tasks, Status::Doing, "Doing:".green());
        println!("-------------------------------");
        print_tasks_by_status(&tasks, Status::Done, "Done:".red());
        println!("====================================");
    }

    Ok(())
}

fn print_tasks_by_status(tasks: &[Task], status: Status, header: ColoredString) {
    println!("{}", header);
    let filtered_tasks: Vec<_> = tasks
        .iter()
        .enumerate()
        .filter(|(_, task)| task.status == status)
        .collect();

    if filtered_tasks.is_empty() {
        println!("No tasks in this category.");
    } else {
        for (index, task) in filtered_tasks {
            println!("{}: {}", index + 1, task);
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}