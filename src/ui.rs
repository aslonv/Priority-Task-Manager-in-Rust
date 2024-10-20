use std::io::{self, Write};

/// Prints the main menu options to the console.
pub fn print_menu() {
    println!("Priority Task Manager");
    println!("1. Add Task");
    println!("2. List Tasks");
    println!("3. Complete Task");
    println!("4. Edit Task");
    println!("5. Remove Task");
    println!("6. Quit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}


/// Reads a line of user input from the console.
/// # Returns
/// A String containing the user's input, with leading and trailing whitespace removed.
pub fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


/// Prompts the user for task details and returns them.
/// # Returns 
/// A tuple containing the task description (String) and priority (u8).
pub fn get_task_details() -> (String, u8) {
    let description = get_string_input("Enter task description: ");
    let priority = get_priority_input("Enter priority (1-5, 5 being highest): ");
    (description, priority)
}


/// Prompts the user for a task ID
///  # Arguments
/// * `prompt` - the message to display when asking for the ID
///  # Returns 
/// The task ID as a u32
pub fn get_task_id(prompt: &str) -> u32 {
    get_number_input(prompt)
}


/// Prompts the user to choose a filter for listing tasks.
/// # Returns 
/// An Option<bool> representing the chosen filter:
/// - None: List all tasks
/// - Some(true): List completed tasks
/// - Some(false): List incompelete tasks
pub fn get_list_filter() -> Option<bool> {
    println!("1. All tasks");
    println!("2. Completed tasks");
    println!("3. Incomplete tasks");

    match get_number_input("Enter your choice: ") {
        1 => None, 
        2 => Some(true),
        3 => Some(false),
        _ => {
            println!("Invalid choice. Listing all tasks.");
            None
        }
    }
}


/// Prompts the user for new task details when editing a task.
/// # Returns 
/// A tuple containing an optional new description (Option<String>) and an optional new priority (Optiona<u8>).
pub fn get_edit_details() -> (Option<String>, Option<u8>) {
    let new_description = get_optional_string_input("Enter new description (leave blank to keep current): ");
    let new_priority = get_optional_priority_input("Enter new priority (1-5, leave blank to keep current): ");
    (new_description, new_priority);
}


/// Helper function to get a string input from the user. 
/// # Arguments
/// * `prompt` - The message to display when asking for input
/// # Returns
/// The user's input as a String. 
fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    get_user_input()
}


/// Helper function to get a number input from the user
/// # Arguments
/// * `prompt` - The message to display when asking for input
/// # Returns
/// The users input as a u32.
fn get_number_input(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        if let Ok(number) = get_user_input().parse() {
            return number;
        }
        println!("Invalid input. Please enter a number.");
    }
}


/// Helper function to get a priority input from the user.
/// # Arguments
/// * `prompt` - The message to display when askin for input
/// # Returns 
/// The user's input as a u8, guaranteed to be between 1 and 5. 
fn get_priority_input(prompt: &str) -> u8 {
    loop {
        let priority = get_number_input(prompt);
        if priority >= 1 && priority <= 5 {
            return priority as u8;
        }
        println!("Invalid priority. Please enter a number between 1 and 5.");
    }
}


/// Helper function to get an optional string from the user.
/// # Arguments
/// * `prompt` - The message to display when asking for input
/// # Returns 
/// An Option<String> containing the user's input, or None if the input was empty
fn get_optional_string_input(prompt: &str) -> Option<String> {
    let input = get_string_input(prompt);
    if input.is_empty() { None } else { Some(input) }
}


/// Helper function to get an optional priority input from the user.
/// # Arguments
/// * `prompt` - The message to display when asking for input
/// # Returns
/// An Option<u8> containing the user's input as a priority (1-5), or None if the input was invalid or emptuy
fn get_optional_priority_input(prompt: &str) -> Option<u8> {
    if let Some(input) = get_optional_string_input(prompt) {
        if let Ok(priority) = input.parse::<u8>() {
            if priority >= 1 && priority <= 5 {
                return Some(priority);
            }
        }
        println!("Invalid priority. Keeping the current priority.");
    }
    None
}