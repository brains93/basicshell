use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const DELIMITER: &str = "<EOF>";

fn start_server() {
    let host = "127.0.0.1";
    let port = 8888;

    let listener = TcpListener::bind((host, port)).expect("Failed to bind to address");
    println!("[*] Listening on {}:{}", host, port);

    if let Ok((mut client_socket, client_address)) = listener.accept() {
        println!("[*] Accepted connection from {}", client_address);

        loop {
            let mut command = String::new();
            println!("Enter command (or 'exit' to quit): ");
            std::io::stdin().read_line(&mut command).expect("Failed to read line");

            let command = command.trim();

            if command.to_lowercase() == "exit" {
                break;
            }

            // Send the command with the delimiter
            client_socket
                .write_all(format!("{}", command).as_bytes())
                .expect("Failed to send command");

            let mut buffer = [0; 1024];
            let bytes_received = client_socket.read(&mut buffer).expect("Failed to read response");

            if bytes_received == 0 {
                break;
            }

            let response = String::from_utf8_lossy(&buffer[..bytes_received]);
            let response = response.trim();

            println!("[*] Response from client:\n{}", response);
        }
    }
}

fn main() {
    start_server();
}
