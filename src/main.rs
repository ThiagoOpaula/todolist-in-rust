use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    checked: bool,
    description: String,
}

fn main() {
    print!(
        "Bem vindo a lista de tarefas!\n
        Lista de opções:
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

        write_json_file(task);
    } else if input.trim() == "2" {
        println!("Listando tarefas");
    } else if input.trim() == "3" {
        println!("Marcar tarefa como concluída");
    } else if input.trim() == "4" {
        println!("Saindo");
    } else {
        println!("Opção inválida");
    }

    //write_json_file();
    //read_json_file();
}

fn read_json_file() {
    let path = Path::new("todo.json");

    let mut file = File::open(&path).expect("Unable to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Unable to read data from file");

    // Deserialize JSON data into Todo struct
    let todo: Vec<Todo> = serde_json::from_str(&json_data).unwrap();
    println!("Todo from JSON file: {:?}", todo);
}

fn write_json_file(task: String) {
    // Create an instance of Todo struct
    let todo: Vec<Todo> = vec![
        Todo {
            checked: false,
            description: String::from("Learn Rust"),
        },
        Todo {
            checked: true,
            description: String::from("duas vezes rust"),
        },
    ];

    // Serialize the Todo struct to JSON
    let json = serde_json::to_string(&todo).unwrap();

    // Write JSON data to a file
    let path = Path::new("todo.json");
    let mut file = File::create(&path).expect("Unable to create file");
    file.write_all(json.as_bytes())
        .expect("Unable to write data to file");

    println!("JSON data has been written to todo.json");
}
