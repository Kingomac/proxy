import socketserver
import socket
import time


def get_response_from_socket(socket: socket.socket, timeout=20):
    start_state = socket.getblocking()
    socket.setblocking(False)
    data = ""
    begin = time.time()
    while True:
        if time.time() - begin < timeout:
            break
        try:
            newdata = socket.recv(2048)
            print("newdata: " + newdata)
            if newdata:
                data += newdata.decode("utf-8")
                begin = time.time()
            else:
                time.sleep(0.1)
        except:
            time.sleep(0.1)
            pass
    socket.setblocking(start_state)
    return data


class MyTCPHandler(socketserver.BaseRequestHandler):

    def get_common(self, lines: list[str]):
        toret = dict[str, str]()
        for line in lines:
            line = line.lower()
            if line.startswith("host: "):
                toret["host"] = line.split(" ")[1]
            elif line.startswith("connection: "):
                toret["connection"] = line.split(" ")[1]
            elif line.startswith("connect") or line.startswith("get") or line.startswith("post"):
                tmp = line.split(" ")
                toret["method"] = tmp[0]
                toret["path"] = tmp[1]
                toret["version"] = tmp[2]
        print("comm:")
        print(toret)
        return toret

    def handle_connect(self, lines: list[str]):
        comm = self.get_common(lines)
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        # s.connect()

    def handle_other(self, lines: list[str]):
        comm = self.get_common(lines)
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        print("connect to " + comm["host"] + ":80")
        s.connect((comm["host"], 80))
        req = "\r\n".join(lines) + "\r\n\r\n"
        s.sendall(req.encode("utf-8"))
        # resp = get_response_from_socket(s)
        print("resp:")
        resp = s.recv(4096)
        print(resp)
        self.request.sendall(resp)

    def handle(self):
        self.data: str = self.request.recv(2048).decode('utf-8').strip()
        print("{} sent: ".format(self.client_address[0]))
        print(self.data)
        print("\n\n")
        if self.data.startswith("CONNECT"):
            self.handle_connect(self.data.split("\r\n"))
        else:
            self.handle_other(self.data.split("\r\n"))


print("Listening on http://0.0.0.0:9090")
with socketserver.TCPServer(("0.0.0.0", 9090), MyTCPHandler) as server:
    server.serve_forever()
