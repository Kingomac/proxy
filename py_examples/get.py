import socket
from time import sleep

host = "localhost"
port = 9090

client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(("127.0.0.1", 9090))
client.sendall(
    # b'GET /online/ HTTP/1.1\r\nHost: funmajesticwonderousplay.neverssl.com\r\nAccept: */*\r\nUser-Agent: xdd\r\n\r\n'
    'GET / HTTP/1.1\r\nHost: www.google.es\r\nAccept: */*\r\nUser-Agent: xdd\r\nAccept-Encoding: identity\r\n\r\n'.encode(
        'utf-8')
)
print("lets wait 20 secs")
sleep(20)
print(client.recv(1024))
