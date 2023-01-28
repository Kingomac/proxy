use super::{ParseError, Parser};
use crate::requests::http_request::{HttpConnection, HttpRequest, HttpRequestTypes};

impl Parser<HttpRequest> for HttpRequest {
    fn from_bytes(bytes: &Vec<u8>) -> Result<HttpRequest, ParseError> {
        todo!()
    }

    fn from_str(text: &str) -> Result<HttpRequest, ParseError> {
        todo!()
    }

    fn from_lines(lines: &Vec<String>) -> Result<HttpRequest, ParseError> {
        let mut result: HttpRequest = HttpRequest::default();
        if lines.len() < 2 {
            println!("ParseError for: {}", lines.join("\n"));
            return Err(ParseError);
        }
        for ele in lines {
            let ele = &ele.to_lowercase();
            if ele.starts_with("host: ") {
                let split: Vec<&str> = ele.split(":").collect();
                let raw_host = split[1];
                result.host = raw_host[1..].to_string();
                if split.len() > 2 {
                    match split[2].parse() {
                        Ok(res) => result.port = res,
                        Err(_) => {
                            println!("ParseError for: {}", lines.join("\n"));
                            return Err(ParseError);
                        }
                    };
                } else {
                    result.port = 443;
                }
            } else if ele.starts_with("connection:") {
                let split: Vec<&str> = ele.split(":").collect();
                let raw_connection = split[1];
                result.connection = match raw_connection {
                    //"close" => HttpConnection::Close,
                    "keep-alive" => HttpConnection::KeepAlive,
                    _ => HttpConnection::Close,
                };
            } else if ele.starts_with("get")
                || ele.starts_with("post")
                || ele.starts_with("put")
                || ele.starts_with("connect")
            {
                let split: Vec<&str> = ele.split(" ").collect();
                println!("🐷🐷🐷:{}", split.join(", "));
                match split[0] {
                    "get" => result.request_type = HttpRequestTypes::GET,
                    "post" => result.request_type = HttpRequestTypes::POST,
                    "put" => result.request_type = HttpRequestTypes::PUT,
                    "connect" => result.request_type = HttpRequestTypes::CONNECT,
                    _ => {
                        println!("Unrecognized HTTP Request Type, setting to default (GET)");
                        result.request_type = HttpRequestTypes::GET
                    }
                }
                result.path = split[1].to_string();
                result.protocol_version = split[2].to_string();
            }
        }
        Ok(result)
    }
}
