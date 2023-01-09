use std::{
    fmt::format,
    io::{prelude::*, BufReader, Read, Write},
    net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    thread,
};

use proxy::{parser::Parser, requests::http_request::HttpRequest};

fn handle_http(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    match HttpRequest::from_lines(&http_request) {
        Ok(parsed) => {
            println!("{}", parsed);
            let proxy_request =
                TcpStream::connect(format!("{}:{}", parsed.host, parsed.port)).unwrap();
            proxy_request.write()
        }
        Err(err) => println!("Couldn't parse request to HTTP: {}", err),
    }

    /*for line in &http_request {
        if line.starts_with("Host: ") {
            let host = line.replace("Host: ", "");
            println!("Read host: {}", host);
            println!("Redirected request -> {}", http_request.join("\r\n"));
            let mut connection = TcpStream::connect(&host).unwrap();
            connection
                .write_all(http_request.join("\r\n").as_bytes())
                .expect("Error writing request to Server");
            let mut response_buf: Vec<u8> = Vec::new();
            connection
                .read_to_end(&mut response_buf)
                .expect("Error reading response");
            println!(
                "Response from {}: {}",
                &host,
                String::from_utf8_lossy(&response_buf)
            );
            stream
                .write_all(&response_buf)
                .expect("Error sending response");
        }
    }*/

    /*let status_line = "HTTP/1.1 200 OK";
    let contents = "<h1>hola</h1>";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();*/
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9090").unwrap();
    println!("Listening on port 9090");
    for req in listener.incoming() {
        //thread::spawn(|| handle_http(req.unwrap()));
        handle_http(req.unwrap())
    }
    /*
    thread::spawn(|| {
        let host = "www.google.es";
        let port = 80;
        println!("Read host: {}", host);
        let mut connection = TcpStream::connect(format!("{}:{}", &host, &port)).unwrap();
        connection
            .write_all(
                format!(
                    "GET / HTTP/1.1\r\nHost: {}:{}\r\nConnection: close\r\n\r\n",
                    host, port
                )
                .as_bytes(),
            )
            .expect("Error writing request to Moovi");
        let mut response_buf: Vec<u8> = Vec::new();
        connection
            .read_to_end(&mut response_buf)
            .expect("Error reading response");
        println!(
            "Response from {}: {}",
            &host,
            String::from_utf8_lossy(&response_buf)
        );
    })
    .join()
    .unwrap();*/
}
