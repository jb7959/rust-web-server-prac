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