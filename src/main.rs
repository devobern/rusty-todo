mod tasks;
use tasks::Task;

use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(_) => Vec::new(),
    };

    let mut index: u32 = 0;
    for task in &tasks {
        if index < task.get_id() {
            index = task.get_id();
        }
    }
    
    loop {
        let mut input = String::new();
        println!("Enter command:");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_parts: Vec<&str> = input.trim().split(' ').collect();
        match input_parts.as_slice() {
            ["add", description]=> {
                index += 1;
                let task:Task = Task::new(index ,description.to_string());
                println!("Task \"{}\" added", task.get_desc());
                tasks.push(task);
            }
            ["list"] => {
                for task in &tasks{
                    task.task_list();
                }
            }
            ["done", id] => {
                if let Ok(parsed_id) = id.parse::<u32>(){
                    match get_task(&mut tasks, parsed_id) {
                        Some(task) => {
                            task.task_done();
                            println!("Task \"{}\" done", task.get_desc());
                        },
                        None => println!("Task not found")
                    }
                } else {
                    println!("Could not parse ID!")
                }
            }
            ["start", id] => {
                if let Ok(parsed_id) = id.parse::<u32>(){
                    match get_task(&mut tasks, parsed_id) {
                        Some(task) => {
                            task.task_start();
                            println!("Task \"{}\" started", task.get_desc());
                        },
                        None => println!("Task not found")
                    }
                } else {
                    println!("Could not parse ID!")
                }
            }
            ["save"] => {
                if let Err(e) = save_tasks(&tasks) {
                    eprintln!("Error saving tasks: {}", e);
                }
            }
            _ => println!("Invalid command.")
        }
    }


}

fn get_task(tasks: &mut Vec<Task>, id: u32) -> Option<&mut Task> {
    for task in tasks {
        if task.get_id() == id{
            return Some(task);
        }
    }
    None
}

fn save_tasks(tasks: &Vec<Task>) -> std::io::Result<()> {
    let json = serde_json::to_string(tasks).unwrap();
    let mut file = File::create("tasks.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn load_tasks() -> std::io::Result<Vec<Task>> {
    let mut file = File::open("tasks.json")?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let tasks: Vec<Task> = serde_json::from_str(&json).unwrap();
    Ok(tasks)
}