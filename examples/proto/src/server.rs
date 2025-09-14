
pub mod proto {
    pub mod hello {
        include!("../generated/proto/hello.rs");
    }
}

use crate::proto::hello::hello::RegisterHelloHandler;
use crate::proto::hello::hello::HelloServer;
use crate::proto::hello::hello::HelloHandler;

use crate::proto::hello::HelloMessage;

use rums::Server;
use rums::RouteHandler;

use std::net::TcpListener;

use std::error::Error;

use std::thread::scope;

struct MyServer {
    id: i32
}

impl HelloServer for MyServer {
    fn hello_world(&mut self, msg: HelloMessage) -> Result<HelloMessage, Box<dyn Error + Send>> {
        println!("server {}: got HelloWorld message: {}", self.id, msg.message);

        let mut msg = HelloMessage::default();
        msg.message = format!("Server {} says Hi", self.id);
        Ok(msg)
    }
    fn goodbye_world(&mut self, msg: HelloMessage) -> Result<HelloMessage, Box<dyn Error + Send>> {
        println!("server {}: got GoodbyeWorld message: {}", self.id, msg.message);

        let mut msg = HelloMessage::default();
        msg.message = format!("Server {} says Bye", self.id);
        Ok(msg)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    scope(|s| {
        for i in 0..10 {
            s.spawn(move || {
                let addr = format!("[::1]:{}", i+50051);
                let listener = TcpListener::bind(addr).unwrap();

                let server_handler = Box::new(HelloHandler::new(
                    Box::new(MyServer{id: i}))
                );

                let mut handler = Box::new(RouteHandler::new());
                handler.register_hello_handler(server_handler);

                let server = Server::new(handler);
                server.serve(listener);
            });
        }
    });

    Ok(())
}
