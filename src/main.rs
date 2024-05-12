use std::process::exit;

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Todo {
    id: Option<i32>,
    description: String,
    checked: bool,
}

fn main() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("todolist.db").expect("failed to open connection");

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             description TEXT NOT NULL,
             checked BOOLEAN NOT NULL default 0
         )",
        [],
    );

    Ok(loop {
        print!(
            "\nWelcome to the to-do list!
        Choose an option:
1. Create a new task
2. View task list
3. Mark task as completed
4. Exit\n
"
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "1" {
            println!("Enter your task:");
            let _ = create_todo(&conn);
        } else if input.trim() == "2" {
            println!("Listing tasks:");
            let _ = read_todos(&conn);
        } else if input.trim() == "3" {
            println!("Enter task id to mark as completed:");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().parse::<i32>().unwrap(); // needs to resign the variable to change the type

            let _ = update_todo_checked(&conn, input, true);
            println!("Marking task as completed:");
        } else if input.trim() == "4" {
            println!("Exiting");
            break;
        } else {
            println!("Invalid option");
        }
    })
}

fn read_todos(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, description, checked FROM todo")?;

    let rows = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            description: row.get(1)?,
            checked: row.get(2)?,
        })
    })?;

    for todo_result in rows {
        match todo_result {
            Ok(todo) => {
                println!("{:?}, {:?}, {:?}", todo.id, todo.description, todo.checked);
            }
            Err(err) => {
                // Handle the error if needed
                eprintln!("Error while retrieving todo: {:?}", err);
            }
        }
    }

    Ok(())
}

fn create_todo(conn: &Connection) -> Result<()> {
    let mut task = String::new();
    let _ = std::io::stdin().read_line(&mut task);

    task = task.trim().to_string();

    let todo = Todo {
        id: None,
        description: task,
        checked: false,
    };

    conn.execute(
        "INSERT INTO todo (description, checked) VALUES (?1, ?2)",
        (todo.description, todo.checked),
    )?;

    Ok(())
}

fn update_todo_checked(conn: &Connection, id: i32, checked: bool) -> Result<()> {
    conn.execute("UPDATE todo SET checked = ?1 WHERE id = ?2", (checked, id))?;
    Ok(())
}
