use anyhow::Result;
use std::fs;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: i32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
        }
    }
}

pub(crate) fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL,
            completed INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub(crate) fn add_task(conn: &Connection, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (description, completed) VALUES (?1, ?2)",
        params![description, false],
    )?;
    println!("Task added: {}", description);
    Ok(())
}

pub(crate) fn list_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    println!("\nTasks:");
    for task in task_iter {
        let task = task?;
        println!("{}: {} [{}]", task.id, task.description, if task.completed { "completed" } else { "pending" });
    }
    Ok(())
}

pub(crate) fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    println!("Task {} deleted.", id);
    Ok(())
}

pub(crate) fn edit_task(conn: &Connection, id: i32, description: &str) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET description = ?1 WHERE id = ?2",
        params![description, id],
    )?;
    println!("Task {} updated.", id);
    Ok(())
}

pub(crate) fn mark_completed(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET completed = ?1 WHERE id = ?2",
        params![true, id],
    )?;
    println!("Task {} marked as completed.", id);
    Ok(())
}

pub(crate) fn save_tasks(conn: &Connection, filename: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, description, completed FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let tasks: Vec<Task> = task_iter.filter_map(rusqlite::Result::ok).collect();
    let serialized = serde_json::to_string(&tasks)?;
    fs::write(filename, serialized)?;
    println!("Tasks saved to {}", filename);
    Ok(())
}

pub(crate) fn load_tasks(conn: &Connection, filename: &str) -> Result<()> {
    let data = fs::read_to_string(filename)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;

    for task in tasks {
        conn.execute(
            "INSERT INTO tasks (id, description, completed) VALUES (?1, ?2, ?3)",
            params![task.id, task.description, task.completed],
        )?;
    }
    println!("Tasks loaded from {}", filename);
    Ok(())
}