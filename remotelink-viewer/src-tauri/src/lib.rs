mod protocol;
mod network;

use protocol::{MouseMove, MouseClick, KeyEvent};
use network::NetworkClient;

// We will store the network client in Tauri state in a real app.
// For now, these commands just print to console.

#[tauri::command]
fn send_mouse_move(x: i32, y: i32) {
    println!("Frontend -> Host: MouseMove {{ x: {}, y: {} }}", x, y);
}

#[tauri::command]
fn send_mouse_click(button: String, down: bool) {
    println!("Frontend -> Host: MouseClick {{ button: {}, down: {} }}", button, down);
}

#[tauri::command]
fn send_key_event(vk: u16, down: bool) {
    println!("Frontend -> Host: KeyEvent {{ vk: {}, down: {} }}", vk, down);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![send_mouse_move, send_mouse_click, send_key_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
