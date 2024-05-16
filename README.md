# Rust Todo List

A simple todo list application written in Rust. Users can add tasks, list tasks, mark tasks as complete, delete tasks, and persist tasks to a file.

## Features

- Add tasks: Users can add new tasks to the todo list.
- List tasks: Users can view all the tasks in the todo list.
- Mark tasks as complete: Users can mark tasks as complete.
- Delete tasks: Users can remove tasks from the todo list.
- Save and load tasks: The todo list is persisted to a file so that tasks are not lost when the program exits.

## Usage

## How to Run

1. Ensure you have Rust installed on your system. If not, you can install it from [Rust's official website](https://www.rust-lang.org/).
2. Clone this repository.
3. Navigate to the project directory in your terminal.
4. Run the following command to compile and run the application:

To use the todo list application, follow these commands:
- $ cargo run
  
- To add a task:
  $ todo add "Finish report"


- To list tasks:
$ todo list


- To mark a task as complete:
$ todo complete <Task index*>


- To delete a task:
$todo delete <task index*>


- To exit the application:
$exit
