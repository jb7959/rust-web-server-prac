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
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
/*        match str::from_utf8(buf) {
            Ok(request) => {

            },
            Err(_) => return Err(ParseError::InvalidEncoding),
        }

        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
            Ok(request) => {}
            Err(e) => return Err(e),
        }*/
        //let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        let request = str::from_utf8(buf)?;
        unimplemented!()
    };
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