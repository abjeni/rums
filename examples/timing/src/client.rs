
use std::string::String;

use rums::Configuration;

use futures::stream::StreamExt;

use futures::select;

pub mod proto {
    pub mod hello {
        include!("../generated/proto/hello.rs");
    }
}

use crate::proto::hello::hello::HelloClient;

use crate::proto::hello::HelloMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut addrs = vec![];

    for i in 0..100 {
        let addr = (String::from(format!("[::1]:{}", 50051+i)), i);
        addrs.push(addr);
    }

    let cfg = Configuration::new(&addrs);

    let mut msg = HelloMessage::default();
    msg.message = "Client says Hello".into();

    let hellos = cfg.hello_world(&msg);
    let mut hellos = hellos.fuse();

    let mut msg = HelloMessage::default();
    msg.message = "Client says Goodbye".into();

    let goodbyes = cfg.goodbye_world(&msg);
    let mut goodbyes = goodbyes.fuse();

    loop {
        select!(
            hello = hellos.next() => {
                if let Some(hello) = hello {
                    match hello.response {
                        Ok(msg) => println!("got HelloWorld response from node {}: {}", hello.node.id, msg.message),
                        Err(e) => println!("HelloWorld response error from node {}: err = {:?}", hello.node.id, e)
                    }
                }
            },
            goodbye = goodbyes.next() => {
                if let Some(goodbye) = goodbye {
                    match goodbye.response {
                        Ok(msg) => println!("got GoodbyeWorld response from node {}: {}", goodbye.node.id, msg.message),
                        Err(e) => println!("GoodbyeWorld response error from node {}: err = {:?}", goodbye.node.id, e)
                    }
                }
            },
            complete => break
        )
    }

    Ok(())
}