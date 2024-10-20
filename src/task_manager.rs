use std::collections::BinaryHeap;
use crate::task::Task;

pub struct TaskManager {
    tasks: BinaryHeap<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: BinaryHeap::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, description: String, priority: u8) -> Result<(), String> {
        let task = Task::new(self.next_id, description, priority)?;
        self.tasks.push(task);
        self.next_id += 1;
        Ok(())
    }

    pub fn list_tasks(&self, filter: Option<bool>) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| filter.map_or(true, |f| task.is_completed() == f))
            .collect()
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), String> {
        self.update_task(id, |task| {
            task.complete();
            Ok(())
        })
    }

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

    pub fn remove_task(&mut self, id: u32) -> Result<(), String> {
        let original_len = self.tasks.len();
        self.tasks.retain(|task| task.id() != id);
        if self.tasks.len() < original_len {
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    }

    fn update_task<F>(&mut self, id: u32, update_fn: F) -> Result<(), String>
    where
        F: FnOnce(&mut Task) -> Result<(), String>,
    {
        let mut task_to_update = None;
        let mut new_heap = BinaryHeap::new();

        while let Some(task) = self.tasks.pop() {
            if task.id() == id {
                task_to_update = Some(task);
                break;
            } else {
                new_heap.push(task);
            }
        }

        // Move remaining tasks to the new heap
        new_heap.extend(self.tasks.drain());

        if let Some(mut task) = task_to_update {
            update_fn(&mut task)?;
            new_heap.push(task);
            self.tasks = new_heap;
            Ok(())
        } else {
            self.tasks = new_heap;
            Err("Task not found".to_string())
        }
    }
}