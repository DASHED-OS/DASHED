use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

// Function to start a proxy server that redirects traffic
pub fn start_proxy_server(listen_addr: &str, redirect_addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(listen_addr)?;
    println!("Proxy server listening on {}", listen_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let redirect_addr = redirect_addr.to_string();
                thread::spawn(move || {
                    handle_client(stream, &redirect_addr);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
    Ok(())
}

// Function to handle client connections and redirect traffic
fn handle_client(mut stream: TcpStream, redirect_addr: &str) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(_) => {
            if let Ok(mut redirect_stream) = TcpStream::connect(redirect_addr) {
                redirect_stream.write_all(&buffer).unwrap();
                let mut response = [0; 512];
                redirect_stream.read(&mut response).unwrap();
                stream.write_all(&response).unwrap();
            } else {
                eprintln!("Failed to connect to redirect address: {}", redirect_addr);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
        }
    }
}

// Example usage
fn main() {
    let listen_addr = "127.0.0.1:8080";
    let redirect_addr = "127.0.0.1:9090";
    if let Err(e) = start_proxy_server(listen_addr, redirect_addr) {
        eprintln!("Error starting proxy server: {}", e);
    }
}