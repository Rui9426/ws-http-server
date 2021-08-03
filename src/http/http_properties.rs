pub mod http_version {
    pub enum HttpVersion {
        Http1_0,
        Http1_1
    }
    pub fn get_version_str(http_version: HttpVersion) -> &'static str {
        match http_version {
            HttpVersion::Http1_0 => {
                return "HTTP/1.0";
            },
            HttpVersion::Http1_1 => {
                return "HTTP/1.1";
            }
        }
    }
}

pub mod http_request_method {
    pub enum HttpRequestMethod {
        GET,POST,DELETE,TRACE,PUT
    }
    pub fn get_http_request_method(http_request_method: HttpRequestMethod) -> &'static str {
        match http_request_method {
            HttpRequestMethod::GET => "GET",
            HttpRequestMethod::POST => "POST",
            HttpRequestMethod::DELETE => "DELETE",
            HttpRequestMethod::TRACE => "TRACE",
            HttpRequestMethod::PUT => "PUT",
        }
    }

}

pub mod http_status_code {
    pub enum HttpStatus {
        OK, 
    }
}
