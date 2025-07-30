
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

use tokio::net::TcpListener;

use futures::stream::FuturesUnordered;

use std::error::Error;

use futures::StreamExt;

struct MyServer {
}

impl HelloServer for MyServer {
    fn hello_world(&self, msg: HelloMessage) -> Result<HelloMessage, Box<dyn Error + Send>> {
        println!("got HelloWorld message: {}", msg.message);
        let mut msg = HelloMessage::default();
        msg.message = "Server says Hi".into();
        Ok(msg)
    }
    fn goodbye_world(&self, msg: HelloMessage) -> Result<HelloMessage, Box<dyn Error + Send>> {
        println!("got GoodbyeWorld message: {}", msg.message);
        let mut msg = HelloMessage::default();
        msg.message = "Server says Bye".into();
        Ok(msg)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut queue = FuturesUnordered::new();

    for i in 0..10 {
        let addr = format!("[::1]:{}", i+50051);
        let listener = TcpListener::bind(addr).await?;

        let server_handler = Box::new(HelloHandler::new(Box::new(MyServer{})));

        let mut handler = Box::new(RouteHandler::new());
        handler.register_hello_handler(server_handler);

        let server = Server::new(handler);
        let srv = server.serve(listener);

        queue.push(srv);
    }

    while let Some(err) = queue.next().await {
        eprintln!("server error {:?}", err)
    }

    Ok(())
}
