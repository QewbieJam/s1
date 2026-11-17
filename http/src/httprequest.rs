use std::collections::HashMap;

// HTTP Method
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "PATCH" => Method::Patch,
            "DELETE" => Method::Delete,
            _ => Method::Uninitialized,
        }
    }
}

// HTTP Version
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Unintialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Unintialized,
        }
    }
}

// HTTP Resource
#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
    QueryString(String),
    Uninitialized,
}

// HTTP Request
#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

// 实现 HttpRequest 的 From trait

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        assert_eq!(Method::from("GET"), Method::Get);
        assert_eq!(Method::from("POST"), Method::Post);
        assert_eq!(Method::from("PUT"), Method::Put);
        assert_eq!(Method::from("PATCH"), Method::Patch);
        assert_eq!(Method::from("DELETE"), Method::Delete);
        assert_eq!(Method::from("INVALID"), Method::Uninitialized);
    }
}



