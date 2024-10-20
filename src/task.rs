use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone)]
pub struct Task {
    id: u32,
    description: String,
    priority: u8,
    completed: bool,
}

impl Task {
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

    pub fn id(&self) -> u32 { self.id }
    pub fn description(&self) -> &str { &self.description }
    pub fn priority(&self) -> u8 { self.priority }
    pub fn is_completed(&self) -> bool { self.completed }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_priority(&mut self, priority: u8) -> Result<(), String> {
        if priority < 1 || priority > 5 {
            return Err("Priority must be between 1 and 5".to_string());
        }
        self.priority = priority;
        Ok(())
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
            .then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}