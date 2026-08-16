pub mod mouse;
pub mod keyboard;
pub mod clipboard;

pub async fn run_input_layer() {
    println!("Initializing Input layer (Agent 2.2)...");
    
    // Spawn the clipboard monitor in background
    tokio::spawn(async {
        clipboard::monitor_clipboard().await;
    });

    println!("Input and Clipboard monitors running.");
}
