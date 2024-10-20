use std::io::{self, Write};

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

pub fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn get_task_details() -> (String, u8) {
    let description = get_string_input("Enter task description: ");
    let priority = get_priority_input("Enter priority (1-5, 5 being highest): ");
    (description, priority)
}

pub fn get_task_id(prompt: &str) -> u32 {
    get_number_input(prompt)
}

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

pub fn get_edit_details() -> (Option<String>, Option<u8>) {
    let new_description = get_optional_string_input("Enter new description (leave blank to keep current): ");
    let new_priority = get_optional_priority_input("Enter new priority (1-5, leave blank to keep current): ");
    (new_description, new_priority)
}

fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    get_user_input()
}

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

fn get_priority_input(prompt: &str) -> u8 {
    loop {
        let priority = get_number_input(prompt);
        if priority >= 1 && priority <= 5 {
            return priority as u8;
        }
        println!("Invalid priority. Please enter a number between 1 and 5.");
    }
}

fn get_optional_string_input(prompt: &str) -> Option<String> {
    let input = get_string_input(prompt);
    if input.is_empty() { None } else { Some(input) }
}

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