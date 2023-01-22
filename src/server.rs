use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    // self를 통해 구조체의 인스턴스를 인자로 받는다.
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept(){
                Ok((stream, _)) => {
                    let a= 5;
                    println!("{}", a);
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }

    }
}