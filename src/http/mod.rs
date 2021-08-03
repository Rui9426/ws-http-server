mod http_properties;
use std::net::TcpListener;

struct HttpServer {
    tcp_listener: TcpListener,
    handler: Handler<HttpRequest>,

}
trait Handler<T> {

    fn handle(&self, t: T);
}
impl Handler<HttpRequest> for HttpServer {
    fn handle(&self, t: HttpRequest) {
        
    }
}
struct HttpRequest {

}