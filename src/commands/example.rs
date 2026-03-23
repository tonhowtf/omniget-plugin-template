use crate::state::MyPluginState;

#[tauri::command]
pub async fn my_command(
    state: tauri::State<'_, MyPluginState>,
    input: String,
) -> Result<String, String> {
    let mut data = state.data.lock().await;
    *data = Some(input.clone());
    Ok(format!("Processed: {}", input))
}
