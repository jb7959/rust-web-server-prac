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