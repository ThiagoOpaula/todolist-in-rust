use std::process::exit;

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Todo {
    id: i32,
    description: String,
    checked: bool,
}

fn main() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("todolist.db").expect("failed to open connection");

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
             id INTEGER PRIMARY KEY,
             description TEXT NOT NULL,
             checked BOOLEAN NOT NULL
         )",
        [],
    );

    let mut stmt = conn.prepare("SELECT id, description, checked FROM todo")?;

    let rows = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            checked: row.get(1)?,
            description: row.get(2)?,
        })
    })?;

    for todo in rows {
        println!("{:?}", todo);
    }

    print!(
        "Bem vindo a lista de tarefas!
        Escolha uma opção:
1. Crie uma nova tarefa
2. Ver lista de tarefas
3. Marcar tarefa como concluída
4. Sair\n
"
    );

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "1" {
        println!("Criando uma nova tarefa");
        let mut task = String::new();
        std::io::stdin().read_line(&mut task).unwrap();
    } else if input.trim() == "2" {
        println!("Listando tarefas");
    } else if input.trim() == "3" {
        println!("Marcar tarefa como concluída");
    } else if input.trim() == "4" {
        println!("Saindo");
    } else {
        println!("Opção inválida");
    }

    Ok(())

    //write_json_file();
    //read_json_file();
}
