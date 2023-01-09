#[derive(Debug)]
pub enum HttpRequestTypes {
    CONNECT,
    GET,
    POST,
    PUT,
    DELETE,
}
#[derive(Debug)]
pub enum HttpConnection {
    KeepAlive,
    Close,
}

pub enum HttpEncoding {
    Deflate,
    GZip,
    Compress,
    Brotli, // BR in HTTP
    Identity,
    All,
}

pub enum HttpCacheControlRequest {
    MaxAge,
    MaxStale,
    MinFresh,
    NoCache,
    NoStore,
    NoTransform,
    OnlyIfCached,
}

pub struct HttpRequest {
    pub request_type: HttpRequestTypes,
    pub host: String,
    pub port: u32,
    pub protocol_version: String,
    pub content_length: u32,
    pub connection: HttpConnection,
    /*user_agent: String,
    accept: String,
    accept_language: String,
    accept_encoding: Vec<HttpEncoding>,
    cache_control: HttpCacheControlRequest,*/
}

impl HttpRequest {
    pub fn default() -> HttpRequest {
        HttpRequest {
            request_type: HttpRequestTypes::CONNECT,
            host: String::new(),
            port: 0,
            protocol_version: String::new(),
            content_length: 0,
            connection: HttpConnection::Close,
        }
    }

    pub fn is_default(&self) -> bool {
        self.port == 0 && self.host.is_empty() && self.protocol_version.is_empty()
    }

    pub fn to_http_string(&self) -> String {
        format!(
            "{:?} {}:{} {}\r\nHost: {}\r\nConnection: {:?}\r\n\r\n",
            self.request_type,
            self.host,
            self.port,
            self.protocol_version,
            self.host,
            self.connection
        )
    }
}

impl std::fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request:\n\t{:?}\n\tVersion: {}\n\tHost: {}\n\tPort: {}\n\tContent-Length: {}\n\tConnection: {:?}\n",
            self.request_type, self.protocol_version, self.host, self.port, self.content_length, self.connection
        )
    }
}

/*
struct HttpRequest {
    request_type: HttpRequestTypes,
    host: String,
    port: u32,
    protocol_version: String,
    content_length: u32,
    connection: HttpConnection,
    /*user_agent: String,
    accept: String,
    accept_language: String,
    accept_encoding: Vec<HttpEncoding>,
    cache_control: HttpCacheControlRequest,*/
}
*/
