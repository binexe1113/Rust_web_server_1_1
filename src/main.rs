fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
struct Server{
    addr: String,
}

impl Server{
    //Constructor for struct Server
    fn new(addr: String)-> Self{
        Server {
            addr 
        }
    }

    //self==this, self takes ownership of struct 
    fn run(self){
        println!("Listening on {}", self.addr);
    }
}

struct Request{
    path: String,
    query_string: Option<String>, //Option cause could be null(Absent value)
    method: Method,
}

enum Method{
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