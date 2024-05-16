use std::fs;
use std::io::{self, Write};
use std::path::Path;

// Define Task struct
#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}

// Define Todo struct
struct Todo {
    tasks: Vec<Task>,
    filename: String,
}

impl Todo {
    fn new(filename: &str) -> Todo {
        let tasks = Todo::load_tasks(filename);
        Todo {
            tasks,
            filename: String::from(filename),
        }
    }

    fn load_tasks(filename: &str) -> Vec<Task> {
        if Path::new(filename).exists() {
            let contents = fs::read_to_string(filename).expect("Error reading file");
            let mut tasks = vec![];
            for line in contents.lines() {
                let task = Task {
                    description: String::from(line),
                    completed: false,
                };
                tasks.push(task);
            }
            tasks
        } else {
            vec![]
        }
    }

    fn save_tasks(&self) -> io::Result<()> {
        let mut file = fs::File::create(&self.filename)?;
        for task in &self.tasks {
            writeln!(file, "{}", task.description)?;
        }
        Ok(())
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!(
                "{}. [{}] {}",
                index + 1,
                if task.completed { "x" } else { " " },
                task.description
            );
        }
    }

    fn complete_task(&mut self, index: usize) {
        if index >= 1 && index <= self.tasks.len() {
            self.tasks[index - 1].completed = true;
            println!("Task marked as complete");
        } else {
            println!("Invalid task index");
        }
    }

    fn delete_task(&mut self, index: usize) {
        if index >= 1 && index <= self.tasks.len() {
            self.tasks.remove(index - 1);
            println!("Task deleted");
        } else {
            println!("Invalid task index");
        }
    }
}

fn main() {
    let mut todo = Todo::new("tasks.txt");

    loop {
        println!("                                                                        ");
        println!("----------------------------------------------------------------------");
        println!("\nWhat would you like to do?");
        println!("- To add a task, type: todo add \"Task description\"");
        println!("- To list tasks, type: todo list");
        println!("- To mark a task as complete, type: todo complete <task index>");
        println!("- To delete a task, type: todo delete <task index>");
        println!("- To exit, type: exit");
        println!("---------------------------------------------------------------------");
        println!("----------------------------------------------------------------------");
        println!("                                                                        ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input_parts: Vec<&str> = input.trim().splitn(3, ' ').collect();

        match input_parts.as_slice() {
            ["todo", "add", description] => {
                let description = description.trim_matches('"');
                todo.add_task(description.to_string());
                todo.save_tasks().expect("Error saving tasks");
                println!("Task added");
            }
            ["todo", "list"] => {
                todo.list_tasks();
            }
            ["todo", "complete", index_str] => {
                let index: usize = match index_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid index");
                        continue;
                    }
                };
                todo.complete_task(index);
                todo.save_tasks().expect("Error saving tasks");
            }
            ["todo", "delete", index_str] => {
                let index: usize = match index_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid index");
                        continue;
                    }
                };
                todo.delete_task(index);
                todo.save_tasks().expect("Error saving tasks");
            }
            ["exit"] => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}
