mod task;
mod task_manager;
mod ui;

use task_manager::TaskManager;
use ui::{print_menu, get_user_input, get_task_details, get_task_id, get_list_filter, get_edit_details};

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        print_menu();
        let choice = get_user_input();

        match choice.trim() {
            "1" => add_task(&mut task_manager),
            "2" => list_tasks(&task_manager),
            "3" => complete_task(&mut task_manager),
            "4" => edit_task(&mut task_manager),
            "5" => remove_task(&mut task_manager),
            "6" => break,
            _ => println!("Invalid choice. Please try again."),
        }

        println!();
    }

    println!("Goodbye!");
}

fn add_task(task_manager: &mut TaskManager) {
    let (description, priority) = get_task_details();
    match task_manager.add_task(description, priority) {
        Ok(_) => println!("Task added successfully."),
        Err(e) => println!("Failed to add task: {}", e),
    }
}

fn list_tasks(task_manager: &TaskManager) {
    let filter = get_list_filter();
    let tasks = task_manager.list_tasks(filter);
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            println!(
                "ID: {}, Priority: {}, Description: {}, Completed: {}",
                task.id(), task.priority(), task.description(), task.is_completed()
            );
        }
    }
}

fn complete_task(task_manager: &mut TaskManager) {
    let id = get_task_id("Enter task ID to mark as completed: ");
    match task_manager.complete_task(id) {
        Ok(_) => println!("Task {} marked as completed.", id),
        Err(e) => println!("Failed to complete task: {}", e),
    }
}

fn edit_task(task_manager: &mut TaskManager) {
    let id = get_task_id("Enter task ID to edit: ");
    let (new_description, new_priority) = get_edit_details();
    if new_description.is_none() && new_priority.is_none() {
        println!("No changes made.");
    } else {
        match task_manager.edit_task(id, new_description, new_priority) {
            Ok(_) => println!("Task {} updated successfully.", id),
            Err(e) => println!("Failed to update task: {}", e),
        }
    }
}

fn remove_task(task_manager: &mut TaskManager) {
    let id = get_task_id("Enter task ID to remove: ");
    match task_manager.remove_task(id) {
        Ok(_) => println!("Task {} removed successfully.", id),
        Err(e) => println!("Failed to remove task: {}", e),
    }
}