use std::collections::HashMap;

#[derive(Clone)]
pub struct AppState {
    database: HashMap<String, String>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            database: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: &String, value: &String) {
        self.database.insert(key.clone(), value.clone());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        match self.database.get(key) {
            None => None,
            Some(x) => Some(x.as_str()),
        }
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
