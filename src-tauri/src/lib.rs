mod wled;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![power_toggle, set_color])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn power_toggle(host: String) -> String {
    wled::send_power(host).await.unwrap_or_else(|e| e)
}

#[tauri::command]
async fn set_color(host: String, r: u8, g: u8, b: u8) -> String {
    let color_vector: (u8, u8, u8) = (r, g, b);
    wled::send_color(host, color_vector)
        .await
        .unwrap_or_else(|e| e)
}
