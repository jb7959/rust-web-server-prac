use std::str::FromStr;

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

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "GET" => OK(Self::GET),
            "DELETE" => OK(Self::DELETE),
            "POST" => OK(Self::POST),
            "PUT" => OK(Self::PUT),
            "HEAD" => OK(Self::PUT),
            "CONNECT" => OK(Self::CONNECT),
            "OPTIONS" => OK(Self::OPTIONS),
            "TRACE" => OK(Self::TRACE),
            "PATCH" => OK(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;