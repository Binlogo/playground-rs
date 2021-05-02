use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Result, Seek, SeekFrom};
use std::path::PathBuf;
use std::{fs::OpenOptions, io::Error};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Self {
        let created_at = Utc::now();
        Task { text, created_at }
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    let mut tasks = parse_tasks(&mut file)?;
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;
    let mut tasks = parse_tasks(&mut file)?;
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invaid task id",
        ));
    }
    tasks.remove(task_position - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    unimplemented!()
}

fn parse_tasks(file: &mut std::fs::File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let mut tasks: Vec<Task> = match serde_json::from_reader(&*file) {
        Ok(tasks) => tasks,
        Err(err) if err.is_eof() => Vec::new(),
        Err(err) => Err(err)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}
