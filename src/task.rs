use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,

    #[serde(default)]
    pub doing: bool,
    pub done: bool,
}

impl Task {
    pub fn new(text: String) -> Self {
        Task {
            text,
            created_at: Utc::now(),
            doing: false,
            done: false,
        }
    }
}

pub fn add_task(journal_path: &PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn doing_task(journal_path: &PathBuf, task_positions: Vec<usize>) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    for &pos in &task_positions {
        if pos == 0 || pos > tasks.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
        }
    }

    for &pos in &task_positions {
        tasks[pos - 1].doing = true;
        tasks[pos - 1].done = false;
    }

    let (done_tasks, incomplete_tasks): (Vec<Task>, Vec<Task>) =
        tasks.into_iter().partition(|task| task.done);

    let (doing_tasks, todo_tasks): (Vec<Task>, Vec<Task>) =
        incomplete_tasks.into_iter().partition(|task| task.doing);

    let mut sorted_tasks: Vec<Task> = todo_tasks;
    sorted_tasks.extend(doing_tasks);
    sorted_tasks.extend(done_tasks);   

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(journal_path)?;

    serde_json::to_writer(file, &sorted_tasks)?;

    Ok(())
}

pub fn done_task(journal_path: &PathBuf, task_positions: Vec<usize>) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    for &pos in &task_positions {
        if pos == 0 || pos > tasks.len() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
        }
    }

    for &pos in &task_positions {
        tasks[pos - 1].doing = false;
        tasks[pos - 1].done = true;
    }

    let (done_tasks, incomplete_tasks): (Vec<Task>, Vec<Task>) =
        tasks.into_iter().partition(|task| task.done);

    let (doing_tasks, todo_tasks): (Vec<Task>, Vec<Task>) =
        incomplete_tasks.into_iter().partition(|task| task.doing);

    let mut sorted_tasks: Vec<Task> = todo_tasks;
    sorted_tasks.extend(doing_tasks);
    sorted_tasks.extend(done_tasks);   

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(journal_path)?;

    serde_json::to_writer(file, &sorted_tasks)?;

    Ok(())
}

pub fn delete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: &PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("{}", "Task list is empty!".yellow());
    } else {
        let mut order: u32 = 1;
        println!("====================================");
        println!("{}", "To Do:".blue());
        for (_index, task) in tasks.iter().enumerate().filter(|(_, task)| !task.doing && !task.done ) {
            println!("{}: {}", order, task);
            order += 1;
        }
        println!("-------------------------------");
        println!("{}", "Doing:".green());
        if tasks.iter().any(|task| task.doing) {
          //  let doing = "[Doing]";
            for (index, task) in tasks.iter().enumerate().filter(|(_, task)| task.doing && !task.done) {
                println!("{}: {}", index + 1, task);
            //    println!("{}: {} {}", index + 1, format!("{}", doing).green(), task);
            }
        }
        println!("-------------------------------");
        println!("{}", "Done:".red());
        if tasks.iter().any(|task| task.done) {
           // let done = "[Done]";
            for (index, task) in tasks.iter().enumerate().filter(|(_, task)| task.done && !task.doing) {
                println!("{}: {}", index + 1, task);
            //    println!("{}: {} {}", index + 1, format!("{}", done).green(), task);
            }
        }
        println!("====================================");
    }

    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}
