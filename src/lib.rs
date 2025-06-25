#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS
}

impl HttpMethod {
    pub fn from_string(s: &str) -> Option<HttpMethod> {
        match s.to_uppercase().as_str() {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "PATCH" => Some(HttpMethod::PATCH),
            "DELETE" => Some(HttpMethod::DELETE),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            _ => None,
        }
    }
}


#[derive(Debug)]
pub enum StatusCode {
    Ok,
    Created,
    NotFound,
    BadRequest,
    InternalServerError,
    MethodNotAllowed,
    //TODO Add more
}

impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::Created => 201,
            StatusCode::BadRequest => 400,
            StatusCode::NotFound => 404,
            StatusCode::MethodNotAllowed => 405,
            StatusCode::InternalServerError => 500,
        }
    }

    pub fn reason_phrase(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

pub enum ContentEncodingType {
    GZIP,
}

impl ContentEncodingType {
    pub fn from_string(s: String) -> Option<ContentEncodingType> {
        match s.to_uppercase().as_str() {
            "GZIP" => {
                Some(ContentEncodingType::GZIP)
            }
            _ => { None }
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            ContentEncodingType::GZIP => Some("gzip".to_string()),
            _ => { None }
        }
    }
}
