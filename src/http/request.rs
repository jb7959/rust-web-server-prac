use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt:: {Display, Debug, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error ;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
// 상호 변환을 위한 구현제를 만들어준다. 예) 암호화 + 복호화
impl TryForm<&[u8]> for Request {
    type Error = String;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        let request = str::from_utf8(buf)?;

/*        match get_next_word(request) {
            Some((method, request)) => {},
            None => return Err(ParseError::InvalidRequest),
        }
*/
        // 로컬 변수 request 이름을 새로 할당했다. 이를 쉐도잉이라고 한다.
        //마지막에 물음표를 안넣으면 예외발생시 강제종료된다.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
    
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    };
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
    };
}

impl Error for ParseError {
    
} 