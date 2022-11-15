use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::env;
use std::process;

#[derive(Debug)]
struct Todo {
    is_done: bool,
    text: String
}

fn get_file(path: &str) -> std::fs::File {
    OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(path)
    .expect(format!("Не вдалось відкрити файл {}", path).as_str())
}

fn check_is_done(raw_todo: &str) -> bool {
    if raw_todo.chars().next().unwrap() == '+' {
        true
    }
    else {
        false
    }
}

fn read_todos(path: &str) -> Vec<Todo> {
    let mut todos_file = get_file(path);
    let mut todos_as_str = String::new();
    todos_file.read_to_string(&mut todos_as_str).expect("Не вдалось прочитати справи");
    let mut todos: Vec<Todo> = Vec::new();
    let raw_todos = todos_as_str.split("\n");
    for raw_todo in raw_todos {
        if raw_todo.len() > 1 {
            let text = &raw_todo[1..raw_todo.len()];
            todos.push(
                    Todo { is_done: check_is_done(raw_todo), text: text.to_string() }
                    );
        }
    }
    todos
}

fn get_write_file(path: &str) -> std::fs::File {
    OpenOptions::new()
    .read(false)
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .expect(format!("Не вдалось відкрити файл {}", path).as_str())
}

fn write_todos(path: &str, todos: Vec<Todo>) {
    let mut todos_file = get_write_file(path);
    let mut base_string = String::new();
    for todo in &todos {
        if todo.is_done {
            base_string.push_str("+");
        }
        else {
            base_string.push_str("-");
        }
        base_string.push_str(todo.text.as_str());
        base_string.push_str("\n");
    }
    let _ = todos_file.write_all(base_string.as_bytes());
    print_todos(todos);
}

fn add_todo(args: Vec<String>, mut todos: Vec<Todo>) -> Vec<Todo>{
    let mut bs = String::new();
    let adds = &args[2..args.len()];
    for add in adds {
        bs.push_str(add.as_str());
        bs.push_str(" ");
    }
    todos.push(
            Todo { is_done: false, text: bs }
            );
    println!("{:?}", todos);
    todos
}

fn fin_todo(args: Vec<String>, mut todos: Vec<Todo>) -> Vec<Todo> {
    let index: usize = args[2].to_string().parse().unwrap();
    todos[index].is_done = true;
    todos
}

fn print_todos(todos: Vec<Todo>) {
    let mut i = 0;
    let todos_len = todos.len();
    while i < todos_len {
        let todo = &todos[i];
        println!("[{}]: ({}) - {}", i, if todo.is_done {"*"} else {" "}, todo.text);
        i += 1;
    }
}

fn rem_todo(args: Vec<String>, mut todos: Vec<Todo>) -> Vec<Todo> {
    let index: usize = args[2].to_string().parse().unwrap();
    todos.remove(index);
    todos
}

fn help() {
    println!("Використання: todo-cli [ПАРАМЕТР] <аргумент>");
    println!("Параметри:");
    println!("  Додати todo:                add <текст>");
    println!("                              a   <текст>\n");
    println!("  Відмітити todo виконаним:   fin <номер>");
    println!("                              f   <номер>\n");
    println!("  Видалити todo               rem <номер>");
    println!("                              r   <номер>");
    println!("                              del <номер>");
    println!("                              d   <номер>");
    println!("\x1b[31m---------------------------------by-Ig4Er");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let todos = read_todos(".todos");
    // write_todos(".todos", todos);
    if args.len() > 1 {
        match args[1].as_str() {
            "add" | "a" => {
                write_todos(".todos", add_todo(args, todos));
            },
            "fin" | "f" => {
                write_todos(".todos", fin_todo(args, todos));
            },
            "rem" | "r" | "del" | "d" => {
                write_todos(".todos", rem_todo(args, todos));
            },
            "--help" | "-h" => {
                help();
            },
            _ => {
                println!("no valid arguments passed, try --help");
                process::exit(0);
            }
        }
    }
    else {
        println!("list todos");
        print_todos(todos);
    }
}
