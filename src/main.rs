fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    }

    // 구조체에 대한 생성자를 만들었다.
    impl Server {
        //fn new (addr: String) -> Server  대신에 Self로 표현이 가능하다.
        pub fn new(addr: String) -> Self {
            Self {
                //addr: addr 를 줄여서 표현가능하다.
                addr
            }
        }

        // self를 통해 구조체의 인스턴스를 인자로 받는다.
        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {
    mod request {
        struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }

    mod method {
        // enum은 각 순서마다 Default는 메모리에 0부터 1씩 증가하며 값을 할당한다.
        // pub가 아니면 private 다.
        pub enum Method {
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
    }
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/