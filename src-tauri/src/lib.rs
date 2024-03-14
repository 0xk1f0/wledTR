mod responses;
mod wled;

use wled::WLEDController;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            power_toggle,
            set_color,
            get_state,
            get_info,
            set_brightness
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn power_toggle(host: String) -> String {
    WLEDController::set_power(host).await.unwrap_or_else(|e| e)
}

#[tauri::command]
async fn set_color(host: String, r: u8, g: u8, b: u8) -> String {
    let color_vector: (u8, u8, u8) = (r, g, b);
    WLEDController::set_color(host, color_vector)
        .await
        .unwrap_or_else(|e| e)
}

#[tauri::command]
async fn get_info(host: String) -> String {
    WLEDController::get_info(host).await.unwrap_or_else(|e| e)
}

#[tauri::command]
async fn get_state(host: String) -> String {
    WLEDController::get_state(host).await.unwrap_or_else(|e| e)
}

#[tauri::command]
async fn set_brightness(host: String, brightness: u8) -> String {
    WLEDController::set_brightness(host, brightness).await.unwrap_or_else(|e| e)
}
