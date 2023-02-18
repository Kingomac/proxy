use std::{
    fmt::format,
    io::{prelude::*, BufReader, Read, Write},
    net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    thread,
};

use proxy::{
    parser::Parser,
    requests::http_request::{HttpRequest, HttpRequestTypes},
};

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
            println!("Opening connection to {}:{}", parsed.host, parsed.port);
            let mut proxy_request =
                TcpStream::connect(format!("{}:{}", parsed.host, parsed.port)).unwrap();
            let response_to_client = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
            loop {
                stream.write_all(response_to_client).unwrap();
                let mut data_from_client: Vec<u8> = Vec::new();
                stream.read_to_end(&mut data_from_client).unwrap();
                if data_from_client.len() == 0 {
                    break;
                }
                println!(
                    "The client sent encrypted data: {}",
                    String::from_utf8_lossy(&data_from_client)
                );
            }
        }
        Err(err) => println!("Couldn't parse request to HTTP: {}", err),
    }
}

fn handle_http2(mut stream: TcpStream) {
    let stream_reader = BufReader::new(&mut stream);
    let request_lines: Vec<_> = stream_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let http_request =
        HttpRequest::from_lines(&request_lines).expect("Error parsing request as HTTP");
    match http_request.request_type {
        HttpRequestTypes::CONNECT => {
            // Connect and create tunnel
            let mut proxy_connection =
                TcpStream::connect(format!("{}:{}", http_request.host, http_request.port))
                    .expect("Error creating proxy connection");
            let response_to_client = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
            stream.write_all(&response_to_client).unwrap();
            loop {
                let mut data_from_client = vec![0; 1500];
                stream.read(&mut data_from_client).unwrap();
                let will_accept = proxy_connection.write(&data_from_client).unwrap(); // 0 means will not accept more data
                if will_accept == 0 {
                    break;
                }
                let mut data_from_server = vec![0; 1500];
                let will_accept = proxy_connection.read(&mut data_from_server).unwrap();
                if will_accept == 0 {
                    break;
                }
                stream.write_all(&data_from_server).unwrap();
            }
        }
        _ => {
            println!("PARSED HTTP REQUEST:");
            println!("TO STRING:");
            println!("{}", http_request.to_http_string());
            println!("EVERYTHING:");
            println!("{}", http_request);
            let mut proxy_req =
                TcpStream::connect(format!("{}:{}", &http_request.host, &http_request.port))
                    .unwrap();
            proxy_req
                .write_all(http_request.to_http_string().as_bytes())
                .unwrap();
            let mut result = String::new();
            proxy_req.read_to_string(&mut result).unwrap();
            println!("DATA FROM SERVER!!:");
            println!("{}", result);
            stream.write_all(&result.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9090").unwrap();
    println!("Listening on port 9090");
    for req in listener.incoming() {
        //thread::spawn(|| handle_http(req.unwrap()));
        thread::spawn(|| handle_http2(req.unwrap()));
    }

    /*thread::spawn(|| {
        /*let host = "www.google.es";
        let port = 80;
        println!("Read host: {}", host);
        let mut connection = TcpStream::connect(format!("{}:{}", &host, &port)).unwrap();
        connection
            .write_all(
                format!(
                    "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                    host
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
        );*/

        let request = HttpRequest {
            host: "google.es".to_string(),
            request_type: HttpRequestTypes::GET,
            port: 80,
            protocol_version: "HTTP/1.1".to_string(),
            content_length: 0,
            connection: proxy::requests::http_request::HttpConnection::Close,
            path: "/".to_string(),
        };

        let mut stream =
            TcpStream::connect(format!("{}:{}", &request.host, &request.port)).unwrap();
        stream
            .write_all(request.to_http_string().as_bytes())
            .unwrap();
        let mut result = String::new();
        stream.read_to_string(&mut result).unwrap();
        println!("req: {}", request.to_http_string());
        println!("xdd\n{}", result);
    })
    .join()
    .unwrap();*/
}
