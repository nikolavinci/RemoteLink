use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;
use tokio::time::{timeout, Duration};

pub async fn handle_session(mut stream: TlsStream<TcpStream>) {
    let mut buf = [0u8; 1024];
    
    // Simulate Heartbeat (30s) and Idle disconnect (120s)
    let idle_timeout = Duration::from_secs(120);
    
    loop {
        match timeout(idle_timeout, stream.read(&mut buf)).await {
            Ok(Ok(0)) => {
                println!("Client disconnected.");
                break;
            }
            Ok(Ok(n)) => {
                println!("Received {} bytes", n);
                // Parse frames here (mock)
            }
            Ok(Err(e)) => {
                eprintln!("Socket error: {}", e);
                break;
            }
            Err(_) => {
                println!("Idle timeout (2 min). Closing connection.");
                break;
            }
        }
    }
}
