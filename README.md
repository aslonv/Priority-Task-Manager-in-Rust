# Priority Task Manager

## Overview

I'm building a simple command-line application in Rust to helps users manage their tasks with priority levels. The tool is intended to be useful in adding tasks, assigning priorities, listing tasks sorted by priority, editing tasks, and marking tasks as completed.

## Features

- Add tasks with descriptions and priority levels
- List tasks sorted by priority
- Edit existing tasks (update description and/or priority)
- Mark tasks as completed
- Remove tasks
- Filter tasks by completion status
- Command-line interface for easy interaction
- Robust error handling for improved reliability

## Installation

To use the Priority Task Manager, you need to have Rust and Cargo installed on your system. If you haven't installed Rust, you can do so by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once Rust is installed, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/priority-task-manager.git
   cd priority-task-manager
   ```

2. Build and run the project:
   ```
   cargo run --release
   ```

## Usage

When you run the application, you'll be presented with a menu of options:

1. Add Task
2. List Tasks
3. Complete Task
4. Edit Task
5. Remove Task
6. Quit

### Adding a Task

To add a task:

1. Choose option 1 from the main menu.
2. Enter the task description when prompted.
3. Enter the priority level (1-5, where 5 is the highest priority) when prompted.

### Listing Tasks

To view tasks:

1. Choose option 2 from the main menu.
2. Select whether to view all tasks, only completed tasks, or only incomplete tasks.
3. Tasks will be displayed with their ID, priority, description, and completion status.

### Completing a Task

To mark a task as completed:

1. Choose option 3 from the main menu.
2. Enter the ID of the task you want to mark as completed when prompted.

### Editing a Task

To edit an existing task:

1. Choose option 4 from the main menu.
2. Enter the ID of the task you want to edit.
3. Enter a new description (or press Enter to keep the current description).
4. Enter a new priority (or press Enter to keep the current priority).

### Removing a Task

To remove a task:

1. Choose option 5 from the main menu.
2. Enter the ID of the task you want to remove.

### Quitting the Application

To exit the application, choose option 6 from the main menu.

## Code Structure

The main components of the Priority Task Manager are:

- `Task`: A struct representing individual tasks with properties like ID, description, priority, and completion status.
- `TaskManager`: A struct that manages the collection of tasks, providing methods to add, list, edit, complete, and remove tasks.
- `ui`: A module handling user interface operations, including input and output.
- Main loop: Handles user interaction, calling appropriate `TaskManager` methods based on user input.

The application uses a `BinaryHeap` to efficiently manage tasks sorted by priority.

## Error Handling

The Priority Task Manager now includes robust error handling. Operations that might fail (such as adding, editing, or removing tasks) return `Result` types, allowing for graceful error recovery and informative error messages to the user.

## Feature Plans for the Future

For future versions, I am considering of adding:

1. Persistent storage: to save tasks to a file so they persist between sessions.
2. Due dates: to add due dates to tasks and sort by both priority and due date.
3. Categories or tags: to implement a system to categorize or tag tasks.
4. User interface improvements:  to implement a text-based user interface (TUI) for a more interactive experience.
5. Task dependencies: to allow tasks to have dependencies on other tasks.
6. Recurring tasks: to add support for tasks that repeat on a schedule.

## Contributing

Contributions to the Priority Task Manager are welcome! Please feel free to submit pull requests or open issues to suggest improvements or report bugs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.