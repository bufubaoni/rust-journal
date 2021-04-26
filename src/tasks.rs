use chrono::{serde::ts_seconds, DateTime, Local, Utc}
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    #[serd(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}


use std::io::Result;
use std::path::PathBuf;

use std::fs::OpenOptions;
use std::io::{BufReader, Result, Seek, SeekFrom};

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()>{
    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(journal_path)?;

    // Consume the file's contents as a vetcor of tasks.
    let mut tasks = Vec<Task> = match serde_json::from_reader(file){
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind the file after reading from it.
    file.seek(SeekFrom::Start(0))?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}
pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()>{
    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(journal_path)?;

    let tasks = match serde_json::from_reader(file){
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    }

    if task_position = 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    file.seek(
        SeekFrom::Start(0)
    ):

    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;

    Ok(())

}
pub fn list_task(journal_path: PathBuf) -> Result<()>{...}

