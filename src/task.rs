use std::cmp::Ordering;

/// Represents a sinle task in the task manager.
/// This struct encapsulates all the information related to a task,
/// including its unique identifier, description, priority, and completion status. 
#[derive(Eq, PartialEq, Clone)]
pub struct Task {
    id: u32,
    description: String,
    priority: u8,
    completed: bool,
}

impl Task {
    /// Creates a new Task with the given id, description, and priority
    /// # Arguments
    /// * `id` - A unique identifier for the task
    /// * `description` - A string describing the task
    /// * `priority` - A number between 1 and 5 indicating the task's priority
    /// Returns a Result containing the new Task if successful, or an error message if the priority is invalid.
    pub fn new(id: u32, description: String, priority: u8) -> Result<Self, String> {
        if priority < 1 || priority > 5 {
            return Err("Priority must be between 1 and 5".to_string());
        }
        Ok(Self {
            id, 
            description,
            priority,
            completed: false,
        })
    }

    // Getter methods
    pub fn id(&self) -> u32 { self.id }
    pub fn description(&self) -> &str { &self.description }
    pub fn priority(&self) -> u8 { self.priority }
    pub fn is_completed(&self) -> bool { self.completed }

    /// Marks the task as completed.
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Updates the task's description
    pub fn set_description(&must self, description: String) {
        self.description = description;
    }

    /// Updates the task's priority
    /// Returns Ok(()) if successful, or an error message if the new priority is invalid
    pub fn set_priority(&mut self, priority: u8) -> Result<(), String> {
        if priority < 1 || priority > 5 {
            return Err("Priority must be between 1 and 5".to_string());
        }
        self.priority = priority;
        Ok(())
    }
}

// These implementations allow Tasks to be compared and ordered,
// which is necessary or storing them in a BinaryHeap.

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by priority (higher priority first)
        other.priority.cmp(&self.priorty)
            // Then by id (lower id first, for stable sorting)
            .then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}