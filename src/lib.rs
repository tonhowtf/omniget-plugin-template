pub mod commands;
pub mod state;

use state::MyPluginState;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("my-plugin")
        .setup(|app, _api| {
            app.manage(MyPluginState::default());
            Ok(())
        })
        .build()
}
