use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

fn arr(a: &[u8]){

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
                Ok((mut stream, _)) => {
                    let a = [1,2,3,4];
                    arr(&a);
                    arr(&a[1..2]); // 이렇게 슬라이스 할 수도 있다.
                    stream.read();
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }

    }
}