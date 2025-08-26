use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server { addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_connection(stream);
                }
                Err(e) => {
                    println!("Failed to establish connection: {}", e);
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request_str = String::from_utf8_lossy(&buffer[..]);

            if let Ok(request) = Request::from(&request_str) {
                println!(
                    "Parsed Request => method: {:?}, path: {}, query: {:?}",
                    request.method, request.path, request.query_string
                );
            } else {
                println!("Failed to parse request:\n{}", request_str);
            }

            // Send a simple HTTP response
            let response = "HTTP/1.1 200 OK\r\n\r\nHello from 
                                                                        Rust!";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from(request: &str) -> Result<Self, String> {
        // First line of HTTP request: "GET /path?query=1 HTTP/1.1"
        let mut lines = request.lines();
        let request_line = lines.next().ok_or("Empty request")?;

        let mut parts = request_line.split_whitespace();
        let method_str = parts.next().ok_or("Missing method")?;
        let path_and_query = parts.next().ok_or("Missing path")?;

        let method = Method::from(method_str)?;

        // Split path and query string
        let mut path_parts = path_and_query.splitn(2, '?');
        let path = path_parts.next().unwrap_or("/").to_string();
        let query_string = path_parts.next().map(|s| s.to_string());

        Ok(Request {
            path,
            query_string,
            method,
        })
    }
}

#[derive(Debug)]
enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl Method {
    fn from(s: &str) -> Result<Self, String> {
        match s {
            "GET" => Ok(Method::GET),
            "DELETE" => Ok(Method::DELETE),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(format!("Unknown method: {}", s)),
        }
    }
}
