# Priority Task Manager

## Overview

Priority Task Manager is a command-line application written in Rust that helps users manage their tasks with priority levels. This simple yet effective tool allows users to add tasks, assign priorities, list tasks sorted by priority, and mark tasks as completed.

## Features

- Add tasks with descriptions and priority levels
- List tasks sorted by priority
- Mark tasks as completed
- Command-line interface for easy interaction

## Installation

To use the Priority Task Manager, you need to have Rust and Cargo installed on your system. If you haven't installed Rust, you can do so by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once Rust is installed, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/priority-task-manager.git
   cd priority-task-manager
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run --release
   ```

## Usage

When you run the application, you'll be presented with a menu of options:

1. Add Task
2. List Tasks (Sorted by Priority)
3. Complete Task
4. Quit

### Adding a Task

To add a task:

1. Choose option 1 from the main menu.
2. Enter the task description when prompted.
3. Enter the priority level (1-5, where 5 is the highest priority) when prompted.

### Listing Tasks

To view all tasks sorted by priority:

1. Choose option 2 from the main menu.
2. Tasks will be displayed with their ID, priority, description, and completion status.

### Completing a Task

To mark a task as completed:

1. Choose option 3 from the main menu.
2. Enter the ID of the task you want to mark as completed when prompted.

### Quitting the Application

To exit the application, choose option 4 from the main menu.

## Code Structure

The main components of the Priority Task Manager are:

- `Task`: A struct representing individual tasks with properties like ID, description, priority, and completion status.
- `TaskManager`: A struct that manages the collection of tasks, providing methods to add, list, and complete tasks.
- Main loop: Handles user interaction, calling appropriate `TaskManager` methods based on user input.

## Feature Plans for the coming future

For the future version, I am considering of adding:

1. Persistent storage: to save tasks to a file so they persist between sessions.
2. Due dates: add due dates to tasks and sort by both priority and due date.
3. Editing tasks: allow users to edit existing task details.
4. Categories or tags: implement a system to categorize or tag tasks.
5. User interface improvements: implement a text-based user interface (TUI) for a more interactive experience.

## Contributing

Contributions to the Priority Task Manager are welcome! Please feel free to submit pull requests or open issues to suggest improvements or report bugs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
