use anyhow::Result;
use rusqlite::{Connection};
use std::{io::{self}};
mod tasks;

fn main() -> Result<()> {
    let conn = Connection::open("tasks.db")?;
    tasks::create_table(&conn)?;

    loop {
        println!("\nChoose an option:");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Delete task");
        println!("4. Edit task");
        println!("5. Mark task as completed");
        println!("6. Save tasks to file");
        println!("7. Load tasks from file");
        println!("8. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        match choice.trim() {
            "1" => {
                let mut description = String::new();
                println!("Enter task description:");
                io::stdin().read_line(&mut description)?;
                tasks::add_task(&conn, description.trim())?;
            }
            "2" => tasks::list_tasks(&conn)?,
            "3" => {
                let mut id = String::new();
                println!("Enter task ID to delete:");
                io::stdin().read_line(&mut id)?;
                tasks::delete_task(&conn, id.trim().parse()?)?;
            }
            "4" => {
                let mut id = String::new();
                let mut description = String::new();
                println!("Enter task ID to edit:");
                io::stdin().read_line(&mut id)?;
                println!("Enter new task description:");
                io::stdin().read_line(&mut description)?;
                tasks::edit_task(&conn, id.trim().parse()?, description.trim())?;
            }
            "5" => {
                let mut id = String::new();
                println!("Enter task ID to mark as completed:");
                io::stdin().read_line(&mut id)?;
                tasks::mark_completed(&conn, id.trim().parse()?)?;
            }
            "6" => {
                let mut filename = String::new();
                println!("Enter filename to save tasks:");
                io::stdin().read_line(&mut filename)?;
                tasks::save_tasks(&conn, filename.trim())?;
            }
            "7" => {
                let mut filename = String::new();
                println!("Enter filename to load tasks from:");
                io::stdin().read_line(&mut filename)?;
                tasks::load_tasks(&conn, filename.trim())?;
            }
            "8" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
    Ok(())
}
