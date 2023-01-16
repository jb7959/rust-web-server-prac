use std::net::TcpListener;

pub struct Server {
    addr: String,
}

// 구조체에 대한 생성자를 만들었다.
impl Server {
    //fn new (addr: String) -> Server  대신에 Self 로 표현이 가능하다.
    pub fn new(addr: String) -> Self {
        Self {
            //addr: addr 를 줄여서 표현가능하다.
            addr
        }
    }

    // self를 통해 구조체의 인스턴스를 인자로 받는다.
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        // 러스트는 Exception Handler를 별도로 지원하지 않는다.
        // 러스트는 복구가능한 에러와 불가능한 에러로 구분하며,
        // 복구가능한 에러의 경우 핸들러 역할을 하는 Result를 지원한다.
        let listener = TcpListener::bind(&self.addr).unwrap();
    }
}