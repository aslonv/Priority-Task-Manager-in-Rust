mod task;
mod task_manager;
mod ui;

use task_manager::TaskManager;
use ui::{print_menu, get_user_input, get_task_details, get_task_id, get_list_filter, get_edit_details};



/// The main function runs the Priority Task Manager application.
/// This function creates a TaskManager instance and enters a loop to handle user interactions.
/// It uses pattern matching to determine which action to take based on user input. 
fn main() {
    // Creates a new instance of TaskManager to store and manage our tasks
    let mut task_manager = TaskManager::new();

    // Main application loop
    loop {
        // Display the menu and get the user's choice
        print_menu();
        let choice = get_user_input();

        // Use pattern matching to handle the user's choice
        match choice.trim() {
            "1" => add_task(&mut task_manager),
            "2" => list_tasks(&task_manager),
            "3" => complete_task(&mut task_manager),
            "4" => edit_task(&mut task_manager),
            "5" => remove_task(&mut task_manager),
            "6" => break, // Exiting the loop and ending the program
            _ => println!("Invalid choice. Please try again."),
        }

        println!(); // Printing a blank line for better readability
    }

    println!("Goodbye!");

}


/// Handles the logic for adding a new task.
/// This function prompts the user for task details and add the task to the TaskManager
fn add_task(task_manager: &mut TaskManager) {
    let (description, priority) = get_task_details();
    match task_manager.add_task(description, priority) {
        Ok(_) => println!("Task added successfully."),
        Err(e) => println!("Failed to add task: {}", e),
    }
}


/// Handles the logic for listing tasks.
/// This function gets a filter from the user and displays the matching tasks. 
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


/// Handles the logic for completing a task.
/// This function prompts the user for a task ID and marks that task as completed. 
fn complete_task(task_manager: &mut TaskManager) {
    let id = get_task_id("Enter task ID to mark as completed: ");
    match task_manager.complete_task(id) {
        Ok(_) => println!("Task {} marked as completed.", id),
        Err(e) => println!("Failed to complete task: {}", e),
    }
}


/// Handles the logic for editing a task. 
/// This function prompts the user for a task ID and new details, then updates the task.
fn edit_task(task_manahger: &mut TaskManager) {
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


/// Handles the logic for removing a task.
/// This function prompts the user for a task ID and removes the task from the TaskManager
fn remove_task(task_manager: &mut TaskManager) {
    let id = get_task_id("Enter task ID to remove: ");
    match task_manager.remove_task(id) {
        Ok(_) => println!("Task {} removed successfully.", id),
        Err(e) => println!("Failed to remove task: {}", e),
    }
}