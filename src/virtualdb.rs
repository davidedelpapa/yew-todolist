use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub todos: Vec<Todo>,
}

impl Database {
    pub fn new() -> Self {
        Database { todos: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Completed,
    Archived,
    Temp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub status: Status,
}

impl Todo {
    pub fn new() -> Self {
        Todo {
            title: "".to_string(),
            description: "".to_string(),
            status: Status::Temp,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.title != ""
    }
}
