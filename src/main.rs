use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

fn main() {
    let port = 8080; // Specify the port number to listen on

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("Server listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut stream_clone = stream.try_clone().unwrap();

    let (tx, rx) = mpsc::channel();

    // Spawn a thread to handle receiving data from the client
    thread::spawn(move || {
        let mut buffer = Vec::new();

        loop {
            let mut chunk = [0; 1024];
            match stream.read(&mut chunk) {
                Ok(0) => {
                    // Connection closed by the client
                    break;
                }
                Ok(n) => {
                    buffer.extend_from_slice(&chunk[..n]);
                    let received_data = String::from_utf8_lossy(&buffer);
                    println!("Received data: {}", received_data);
                    buffer.clear();
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    });

    // Spawn a thread to handle sending data to the client
    thread::spawn(move || loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let response = format!("{}", input);
        stream_clone.write(response.as_bytes()).unwrap();
        stream_clone.flush().unwrap();

        tx.send(()).unwrap(); // Signal that the response has been sent
    });

    // Wait for the sending thread to finish
    rx.recv().unwrap();
}
