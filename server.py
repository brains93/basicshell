import socket

def start_server():
    host = '127.0.0.1' #change to your network IP that the client can see
    port = 8888

    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind((host, port))
    server_socket.listen(1)

    print(f"[*] Listening on {host}:{port}")

    client_socket, client_address = server_socket.accept()
    print(f"[*] Accepted connection from {client_address}")

    while True:
        command = input("Enter command (or 'exit' to quit): ")
        if command.lower() == 'exit':
            break

        client_socket.send(command.encode())
        response = client_socket.recv(1024).decode()
        print(f"[*] Response from client: {response}")

    server_socket.close()

if __name__ == "__main__":
    start_server()
