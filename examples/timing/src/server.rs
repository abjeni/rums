
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
use std::thread::sleep;

use std::time::Duration;

use rand::Rng;
use rand;

struct MyServer {
    id: i32
}

impl HelloServer for MyServer {
    fn hello_world(&mut self, msg: HelloMessage) -> Result<HelloMessage, Box<dyn Error + Send>> {
        println!("server {}: got HelloWorld message: {}", self.id, msg.message);

        let mut rng = rand::rng();
        let millis = rng.random_range(0..10000);
        sleep(Duration::from_millis(millis));

        let mut msg = HelloMessage::default();
        msg.message = format!("Server {} says Hi after {}ms", self.id, millis);
        Ok(msg)
    }
    fn goodbye_world(&mut self, msg: HelloMessage) -> Result<HelloMessage, Box<dyn Error + Send>> {
        println!("server {}: got GoodbyeWorld message: {}", self.id, msg.message);

        let mut rng = rand::rng();
        let millis = rng.random_range(0..10000);
        sleep(Duration::from_millis(millis));

        let mut msg = HelloMessage::default();
        msg.message = format!("Server {} says Bye after {}ms", self.id, millis);

    

        Ok(msg)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    scope(|s| {
        for i in 0..100 {
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
