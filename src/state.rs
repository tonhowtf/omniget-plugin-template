use std::sync::Arc;

pub struct MyPluginState {
    pub data: Arc<tokio::sync::Mutex<Option<String>>>,
}

impl Default for MyPluginState {
    fn default() -> Self {
        Self {
            data: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }
}
