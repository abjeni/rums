
use std::string::String;

use rums::Configuration;
use rums::SendOptions;
use rums::add_route;

use std::io::Result;

use std::io::ErrorKind;

#[tokio::main]
async fn main() -> Result<()> {

    let mut addrs = vec![];

    for i in 0..10 {
        let addr = (String::from(format!("[::1]:{}", 50051+i)), i);
        addrs.push(addr);
    }

    let cfg = Configuration::new(&addrs);

    let data = Vec::from("hello, world!");
    let data = add_route(data, "world");
    let data = add_route(data, "hello");

    let responses = cfg.send(data, SendOptions::default())?;


    for res in responses {
        match res.response {
            Ok(msg) => {
                match String::from_utf8(msg) {
                    Ok(text) => println!("got response from node {}: {}", res.node.unwrap().id, text),
                    Err(e) => println!("node {}: response not utf8: err = {:?}", res.node.unwrap().id, e)
                }
            },
            Err(e) => {
                println!("response error: err = {:?}", e);
                if e.kind() == ErrorKind::NotFound {
                    break;
                }
            }
        }
    }

    Ok(())
}