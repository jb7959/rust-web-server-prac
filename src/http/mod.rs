// mod.rs의 경우에는 Facade 패턴과 유사하다.
pub use request::Request; // 이렇게 해야 사용하는 쪽에서 use http::Request 와 같이 곧바로 사용 할 수 있다.
pub use request:: ParseError;
pub use method::Method;

pub mod request; // 이렇게 해야 사용하는 쪽에서 use http::request::Request 와 같이 사용 할 수 있다
pub mod method;
