use chrono::{DateTime, Utc, Local, serde::ts_seconds};
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::io::{Result, SeekFrom, Seek, ErrorKind, Error};
use std::fs::{OpenOptions, File};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at = Utc::now();
        Task {
            text,
            created_at,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        //{:<50}: a left-aligned string padded with 50 spaces.
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}


pub fn add_task(joural_path: PathBuf, task: Task) -> Result<()> {
    let file = file_open(joural_path)?;

    let mut tasks = collect_tasks(&file)?;

    tasks.push(task);
    write_tasks(file, &mut tasks)?;
    Ok(())
}

fn file_open(joural_path: PathBuf) -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(joural_path)?;
    Ok(file)
}

pub fn compare_task(joural_path: PathBuf, task_position: usize) -> Result<()> {
    // open file
    let file = file_open(joural_path)?;
    let mut tasks = collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    tasks.remove(task_position - 1);

    write_tasks(file, &mut tasks)?;
    Ok(())
}

fn write_tasks(file: File, tasks: &mut Vec<Task>) -> Result<()> {
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(joural_path: PathBuf) -> Result<()> {
    let file = file_open(joural_path)?;
    let tasks: Vec<Task> = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("No any task");
    } else {
        let mut order = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}
