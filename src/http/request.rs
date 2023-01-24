use super::method::{Method, MethodError};
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

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        // 1. match 쓰려고하나 None의 경우 필요가 없음에도 선언해야함 (불필요한 빈매치)
        let method: Method = method.parse()?;
        let mut query_string = None;
        match path.find('?') {
            Some(i) => {
                query_string = Some(&path[i + 1..]);
                path = &path[..i]
            }
            None => {

            }
        }

        // 2. match 를 사용하지 않으면 if가 등장함 + 불필요한 변수 필요
        let q = path.find('?');
        if(q.is_some()){
            let i = q.unwrap();
            query_string = Some(&path[i + 1..]);
            path = &path[..i]
        }

        // 3. rust에는 if let이라는 것이 있음, 불필요한 매치 제거 및 변수도 제거, 깔끔해짐
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i]
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

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self {
        Self::InvalidEncoding
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