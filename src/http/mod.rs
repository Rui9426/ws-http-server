use std::error::Error;
pub mod protocol;
mod obj {
    use std::{alloc::System, net::{TcpListener, TcpStream}};

    use crate::logging::debug;

    pub struct HttpRequest {}

    pub trait InBoundHandler<T> {
        fn handle(&self, t: T);
    }

    struct ThreadPool {}

    impl ThreadPool {
        fn execute(&self) {}

        fn new(size: usize) -> ThreadPool {
            ThreadPool {

            }
        }
    }


    pub struct HttpServer {
        listener: TcpListener,
        thread_pool: ThreadPool,
    }

    impl HttpServer {
        pub fn accept<T: FnOnce(TcpStream) + Copy>(&self, handler: T) {
            debug("Running TcpLitener...", crate::logging::LEVEL::DEBUG);
            for stream in self.listener.incoming() {
                let stream = stream.unwrap();
                // handler.handle(stream);
                handler(stream);
                debug("Connection established", crate::logging::LEVEL::DEBUG);
            }
        }

        pub fn new(addr: &str) -> HttpServer {
            let tcp_listener = TcpListener::bind(addr).unwrap();
            HttpServer {
                listener: tcp_listener,
                thread_pool: ThreadPool::new(8),
            }
        }
    }
}

use crate::http::obj::HttpServer;
pub fn create_http_server(addr: &str) -> Result<HttpServer, Box<dyn Error>> {
    return Ok(HttpServer::new(addr));
}
