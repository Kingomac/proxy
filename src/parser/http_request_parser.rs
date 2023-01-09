use super::{ParseError, Parser};
use crate::requests::http_request::{HttpRequest, HttpRequestTypes};

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
            return Err(ParseError);
        }
        for ele in lines {
            let ele = &ele.to_lowercase();
            if ele.starts_with("host: ") {
                let split: Vec<&str> = ele.split(":").collect();
                result.host = split[0].to_string();
                match split[1].parse() {
                    Ok(res) => result.port = res,
                    Err(_) => return Err(ParseError),
                };
            } else if ele.starts_with("get") || ele.starts_with("post") || ele.starts_with("put") {
                let split: Vec<&str> = ele.split(" ").collect();
                match split[0] {
                    "get" => result.request_type = HttpRequestTypes::GET,
                    "post" => result.request_type = HttpRequestTypes::POST,
                    "put" => result.request_type = HttpRequestTypes::PUT,
                    _ => {
                        println!("Unrecognized HTTP Request Type, setting to default (GET)");
                        result.request_type = HttpRequestTypes::GET
                    }
                }
                result.protocol_version = split[2].to_string();
            }
        }
        Ok(result)
    }
}
