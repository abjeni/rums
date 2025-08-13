

use std::net::TcpListener;

use std::error::Error;

use std::thread::scope;

use rums::Server;
use rums::ServerHandler;
use rums::RouteHandler;

struct MyServer {
    id: i32
}

impl ServerHandler for MyServer {
    fn handle(&mut self, request: &[u8]) -> Result<Vec<u8>, Box<dyn Error + Send>> {
        println!("server {}: got request: {}", self.id, String::from_utf8_lossy(request));

        Ok(Vec::from(format!("hi from server {}", self.id)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    scope(|s| {
        for i in 0..100 {
            s.spawn(move || {
                let addr = format!("[::1]:{}", i+50051);
                let listener = TcpListener::bind(addr).unwrap();

                let server = MyServer{id: i};

                let mut handler = Box::new(RouteHandler::new());
                handler.sub_route("hello".into()).add_route("world".into(), Box::new(server));

                let server = Server::new(handler);
                server.serve(listener);
            });
        }
    });

    Ok(())
}