mod capture;
mod encoder;
mod network;

fn main() {
    encoder::init();
    network::init();

    println!("Starting RemoteLink Host - Desktop Capture Module");
    
    if let Err(e) = capture::run_capture_loop() {
        eprintln!("Capture loop exited with error: {:?}", e);
    }
}
