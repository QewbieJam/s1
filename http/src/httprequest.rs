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
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Unintialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";


        let mut request_line_parsed = false;
        for line in req.lines() {
            if !request_line_parsed {
                // The request line comes first; ignore anything before it.
                if line.contains("HTTP") {
                    let (method, resource, version) = process_req_line(line);
                    parsed_method = method;
                    parsed_resource = resource;
                    parsed_version = version;
                    request_line_parsed = true;
                }
                continue;
            }

            if line.is_empty() {
                // Empty line indicates the end of headers and start of body
                parsed_msg_body = req
                    .split_once("\r\n\r\n")
                    .or_else(|| req.split_once("\n\n"))
                    .map(|(_, body)| body)
                    .unwrap_or("");
                break;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else {
                parsed_msg_body = line;
            }
        }
        // Return the parsed HttpRequest
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.trim().to_string(); 
    }
    if let Some(v) = header_items.next() {
        value = v.trim().to_string();
    }
    (key, value)
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

    #[test]
    fn test_parses_request_line_headers_and_body() {
        let request = String::from(
            "POST /users HTTP/1.1\r\n\
             Host: localhost\r\n\
             Content-Type: application/json\r\n\
             \r\n\
             {\"name\": \"Ada\"}",
        );
        let parsed = HttpRequest::from(request);

        assert_eq!(parsed.method, Method::Post);
        assert_eq!(parsed.version, Version::V1_1);
        assert_eq!(parsed.resource, Resource::Path("/users".to_string()));
        assert_eq!(parsed.headers["Host"], "localhost");
        assert_eq!(parsed.headers["Content-Type"], "application/json");
        assert_eq!(parsed.msg_body, "{\"name\": \"Ada\"}");
    }

    #[test]
    fn test_invalid_request_line_keeps_defaults() {
        let parsed = HttpRequest::from(String::from("INVALID"));

        assert_eq!(parsed.method, Method::Uninitialized);
        assert_eq!(parsed.version, Version::Unintialized);
        assert_eq!(parsed.resource, Resource::Path(String::new()));
        assert_eq!(parsed.msg_body, String::new());
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from(
            "GET /greeting HTTP/1.1\r\n\
             Host: localhost\r\n\
             Accept: */*\r\n\
             User-Agent: Rustrover/1.0\r\n\
             \r\n",
        );
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("User-Agent".into(), "Rustrover/1.0".into());

        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }

}
