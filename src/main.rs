use server::Server;
use http::request::Request;
use http::method::Method;

mod server;
mod http; // http.rs 혹은 http/mod.rs를 모듈로 대입한다.

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/