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