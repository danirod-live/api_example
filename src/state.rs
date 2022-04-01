use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub enum Status {
    Received,
    Executing,
    Done(#[serde(skip_serializing)] String),
    Error(#[serde(skip_serializing)] String),
}

#[derive(Clone, Debug)]
pub struct Task {
    pub command: String,
    pub status: Status,
}

#[derive(Clone)]
pub struct AppState {
    database: HashMap<String, Task>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            database: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: &String, command: &String) {
        let task = Task {
            command: command.clone(),
            status: Status::Error("jasklasjhlsdk".to_string()),
        };
        self.database.insert(key.clone(), task);
    }

    pub fn get(&self, key: &str) -> Option<&Task> {
        self.database.get(key)
    }

    pub fn length(&self) -> usize {
        self.database.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_is_empty() {
        let state = AppState::new();
        assert_eq!(state.length(), 0);
    }

    #[test]
    fn test_app_can_put_and_get() {
        let mut state = AppState::new();
        state.put(&"holiwi".to_string(), &"hola mundo".to_string());
        assert_eq!(state.get("holiwi"), Some("hola mundo"));
        assert_eq!(state.get("nope"), None);
        assert_eq!(state.length(), 1);
        assert_eq!(state.get("holiwi"), Some("hola mundo"));
    }
}
