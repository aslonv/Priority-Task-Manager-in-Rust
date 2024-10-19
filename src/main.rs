use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, Write};

/// Represents a single task in the task manager.
#[derive(Eq, PartialEq)]
struct Task {
    id: u32,
    description: String,
    priority: u8,
    completed: bool,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by priority (higher priority first)
        other.priority.cmp(&self.priority)
            // Then by id (lower id first, for stable sorting)
            .then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Manages a collection of tasks, providing methods to add, list, and complete tasks.
struct TaskManager {
    tasks: BinaryHeap<Task>,
    next_id: u32,
}

impl TaskManager {
    /// Creates a new, empty TaskManager.
    fn new() -> Self {
        TaskManager {
            tasks: BinaryHeap::new(),
            next_id: 1,
        }
    }

    /// Adds a new task to the manager with the given description and priority.
    fn add_task(&mut self, description: String, priority: u8) {
        let task = Task {
            id: self.next_id,
            description,
            priority,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("Task added successfully.");
    }

    /// Lists all tasks, sorted by priority.
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            let sorted_tasks: Vec<_> = self.tasks.iter().collect();
            for task in sorted_tasks {
                println!(
                    "ID: {}, Priority: {}, Description: {}, Completed: {}",
                    task.id, task.priority, task.description, task.completed
                );
            }
        }
    }

    /// Marks the task with the given ID as completed.
    fn complete_task(&mut self, id: u32) {
        let mut completed_task: Option<Task> = None;
        self.tasks = self.tasks.drain().filter_map(|mut task| {
            if task.id == id {
                task.completed = true;
                completed_task = Some(task);
                None
            } else {
                Some(task)
            }
        }).collect();

        if let Some(task) = completed_task {
            println!("Task {} marked as completed.", id);
            self.tasks.push(task);
        } else {
            println!("Task not found.");
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        print_menu();
        let choice = get_user_input();

        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let description = get_user_input();
                print!("Enter priority (1-5, 5 being highest): ");
                io::stdout().flush().unwrap();
                if let Ok(priority) = get_user_input().trim().parse() {
                    if priority >= 1 && priority <= 5 {
                        task_manager.add_task(description, priority);
                    } else {
                        println!("Invalid priority. Please enter a number between 1 and 5.");
                    }
                } else {
                    println!("Invalid priority. Please enter a number.");
                }
            }
            "2" => task_manager.list_tasks(),
            "3" => {
                print!("Enter task ID to mark as completed: ");
                io::stdout().flush().unwrap();
                if let Ok(id) = get_user_input().trim().parse() {
                    task_manager.complete_task(id);
                } else {
                    println!("Invalid ID. Please enter a number.");
                }
            }
            "4" => break,
            _ => println!("Invalid choice. Please try again."),
        }

        println!();
    }

    println!("Goodbye!");
}

/// Prints the main menu options.
fn print_menu() {
    println!("Priority Task Manager");
    println!("1. Add Task");
    println!("2. List Tasks (Sorted by Priority)");
    println!("3. Complete Task");
    println!("4. Quit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

/// Gets user input from the command line.
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}