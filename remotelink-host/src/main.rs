mod capture;
mod encoder;
mod network;
mod input;

#[tokio::main]
async fn main() {
    encoder::init();
    
    tokio::spawn(async {
        input::run_input_layer().await;
    });
    
    // Spawn network server concurrently
    tokio::spawn(async {
        if let Err(e) = network::run_server().await {
            eprintln!("Network server error: {}", e);
        }
    });

    println!("Starting RemoteLink Host - Desktop Capture Module");
    
    // Capture loop blocks the main thread
    if let Err(e) = capture::run_capture_loop() {
        eprintln!("Capture loop exited with error: {:?}", e);
    }
}
