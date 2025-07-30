use tokio::net::TcpListener;

use futures::{
    stream::FuturesUnordered,
    StreamExt,
};

use std::error::Error;

use rums::Server;
use rums::RouteHandler;

fn my_handler(request: &[u8]) -> Vec<u8> {
    println!("got request: {}", String::from_utf8_lossy(request));
    Vec::from("hi to client")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut queue = FuturesUnordered::new();

    for i in 0..10 {
        let addr = format!("[::1]:{}", i+50051);
        let listener = TcpListener::bind(addr).await?;

        let mut handler = Box::new(RouteHandler::new());
        handler.sub_route("hello".into()).add_route("world".into(), Box::new(my_handler as for<'a> fn(&'a [u8]) -> Vec<u8>));

        let server = Server::new(handler);
        let srv = server.serve(listener);

        queue.push(srv);
    }

    while let Some(err) = queue.next().await {
        eprintln!("server error {:?}", err)
    }

    Ok(())
}