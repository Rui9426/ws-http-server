use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
};

use logging::debug;

mod http;
mod logging;

use crate::http::protocol::resp;
fn main() {
    let http = http::create_http_server("localhost:8080").unwrap();

    http.accept(|mut stream| {
        let mut buffer = [0u8; 1024];
        stream.read(&mut buffer).unwrap();
        let resp_line = resp::get_ok_resp_status_line();
        let content = fs::read_to_string("index.html").unwrap();

        stream
            .write(
                format!(
                    "{0}Content-Length: {1}\r\n\r\n{2}",
                    resp_line,
                    content.len(),
                    content
                )
                .as_bytes(),
            )
            .unwrap();
        stream.flush().unwrap();
    });
    debug("Hello", logging::LEVEL::INFO)
}
