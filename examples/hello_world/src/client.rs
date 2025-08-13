
use std::string::String;

use rums::Configuration;
use rums::add_route;

use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut addrs = vec![];

    for i in 0..10 {
        let addr = (String::from(format!("[::1]:{}", 50051+i)), i);
        addrs.push(addr);
    }

    let cfg = Configuration::new(&addrs);

    let data = Vec::from("hello, world!");
    let data = add_route(data, "world");
    let data = add_route(data, "hello");

    let mut responses = cfg.send(data);

    while let Some(res) = responses.next().await {
        match res.response {
            Ok(msg) => {
                match String::from_utf8(msg) {
                    Ok(text) => println!("got response from node {}: {}", res.node.id, text),
                    Err(e) => println!("node {}: response not utf8: err = {:?}", res.node.id, e)
                }
            },
            Err(e) => println!("node {}: response error: err = {:?}", res.node.id, e)
        }
    }

    Ok(())
}