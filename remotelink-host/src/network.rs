use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use std::sync::Arc;

pub mod tls;
pub mod server;
pub mod frame;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Network layer (Agent 2.1)...");
    
    // Generate or load TLS certs
    let (cert, key) = tls::generate_self_signed_cert()?;
    let config = tls::create_server_config(cert, key)?;
    let acceptor = TlsAcceptor::from(Arc::new(config));

    // Bind TCP Listener
    let listener = TcpListener::bind("0.0.0.0:5900").await?;
    println!("Server listening on TCP 0.0.0.0:5900 (TLS 1.3 only)");

    // Accept only ONE connection at a time
    loop {
        let (stream, peer_addr) = listener.accept().await?;
        println!("Accepted TCP connection from {}", peer_addr);

        match acceptor.accept(stream).await {
            Ok(tls_stream) => {
                println!("TLS Handshake completed with {}", peer_addr);
                // Enter session loop
                server::handle_session(tls_stream).await;
                println!("Session ended. Waiting for new connection...");
            }
            Err(e) => {
                eprintln!("TLS Handshake failed: {}", e);
            }
        }
    }
}

pub fn init() {
    println!("Network stub replaced with fully functional module.");
}
