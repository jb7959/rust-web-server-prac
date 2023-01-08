fn main() {
    let string = String::from("127.0.0.1:8080"); // heap에 할당함
    let string_slice = &string[10..]; //문자열 차용 -> 포인터의 참조범위를 인덱스를 통해서 자름

    let emoji = "😁🚀😍😓🚗"; // -> RUST가 채용하는 UTF8에 의해서 일본어나 이모지등은 2바이트 이상이라서 문제가 될 수 있다.
    let emoji_slice = &emoji[..4]; //- 😁🚀 출력을 기대

    let string_borrow = &string; // 차용
    let string_literal = "1234"; // 리터럴 -> 힙에 할당
    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);

    dbg!(emoji);
    dbg!(emoji_slice);//- 😁 출력만 함. 이모지는 4byte 임

    // let server = Server::new("127.0.0.1:8080");
   // server.run();
}

struct Server {
    addr: String,
}

// 구조체에 대한 생성자를 만들었다.
impl Server {
    //fn new (addr: String) -> Server  대신에 Self로 표현이 가능하다.
    fn new (addr: String) -> Self {
        Self {
            //addr: addr 를 줄여서 표현가능하다.
            addr
        }
    }

    // self를 통해 구조체의 인스턴스를 인자로 받는다.
    fn run(self) {

    }
}