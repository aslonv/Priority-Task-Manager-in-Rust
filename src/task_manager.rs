use std::collections::BinaryHeap;
use crate::task::Task;

/// Manages a collection of tasks, providing methods to add, list, edit, and remove tasks.
pub struct TaskManager {
    // BinaryHeap is used to keep tasks sorted by priority
    tasks: BinaryHeap<Task>,
    // Keeps tracks of the net available task ID
    next_id: u32, 
}

impl TaskManager {
    /// Creates a new, empty TaskManager.
    pub fn new() -> Self {
        Self {
            tasks: BinaryHeap::new(),
            next_id: 1,
        }
    } 

    /// Adds a new task to the manager with the given description and priority.
    /// # Returns
    /// Returns Ok(()) if the task was added successfully, or an error message if the priority is invalid. 
    pub fn add_task(&mut self, description: String, priority: u8) -> Result<(), String> {
        let task = Task::new(self.next_id, description, priority)?;
        self.tasks.push(task);
        self.next_id += 1;
        Ok(())
    }

    /// Lists all tasks, optionally filtered by completion status.
    /// # Arguments
    /// * `filter` - An optional boolean. If Some(true), only completed tasks are returned.
    ///             If Some(false), only incomplete tasks are returned. If None, all tasks are returned.
    /// # Returns
    /// A vector of references to Task objects matching the filter criteria.
    pub fn list_tasks(&self, filter: Option<bool>) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| filer.map_or(true, |f| task.is_completed() == f))
            .collect()
    }  

    /// Marks the task with the given ID as completed.
    /// # Returns
    /// Returns Ok(()) if the task was completed successfully, or an error message if the task was not found.
    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        self.update_task(id, |task| task.complete())
    } 

    /// Edits the task with the given ID, updating its description and/or priority.
    /// # Arguments
    /// * `id` - The ID of the task to edit
    /// * `new_description` - An optional new description for the task
    /// * `new_priority` - An optional new priority for the task
    /// # Returns
    /// Returns Ok(()) if the task was edited successfully, or an error message if the task was not found
    /// or the if the new priority is invalid. 
    pub fn edit_task(&mut self, id: u32, new_description: Option<String>, new_priority: Option<u8>) -> Result<(), String> {
        self.update_task(id, |task| {
            if let Some(desc) = new_description {
                task.set_description(desc);
            }
            if let Some(prio) = new_priority {
                task.set_priority(prio)?;
            }
            Ok(())
        })
    }

    /// Removes the task with the given ID from the manager.
    /// Returns Ok(()) if the task was removed successfully, or an error message if the task was not found.
    pub fn remove_task(&mut self, id: u32) -> Result<(), String> {
        let original_len = self.tasks.len();
        self.tasks.retain(|task| task.id() != id);
        if self.tasks.len() < original_len {
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    /// Helper method to update a task with a given update function.
    /// # Arguments
    /// * `id` - The ID of the task to update
    /// * `update_fn` - A closure that takes a mutable reference to a Task and returns a Result
    /// Returns Ok(()) if the task was updated successfully, or an error message if the task was not found
    /// or if the update function returns an error.
    fn update_task<F>(&mut self, id: u32, update_fn: F) -> Result<(), String>
    where 
        F: Fn(&mut Task) -> Result<(), String>,
    {
        let mut updated_task: Option<Task> = None;
        self.tasks = self.tasks.drain().filter_map(|mut task| {
            if task.id() == id {
                if let Err(e) = update_fn(&mut task) {
                    return Some(Err(e));
                }
                updated_task = Some(task.clone());
                Some(Ok(task))
            } else {
                Some(Ok(task))
            }
        }).collect::<Result<BinaryHeap<_>, _>>()?;

        if updated_task.is_some() {
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    } 
 }