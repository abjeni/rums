
use std::string::String;

use rums::Configuration;
use rums::Responses;
use rums::Selection;
use rums::SendOptions;

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

    for i in 0..10 {
        let addr = (String::from(format!("[::1]:{}", 50051+i)), i);
        addrs.push(addr);
    }

    let cfg = Configuration::new(&addrs);

    let mut msg = HelloMessage::default();
    msg.message = "Client says Hello".into();

    let mut hellos = cfg.hello_world(&msg, SendOptions::default())?;

    let mut msg = HelloMessage::default();
    msg.message = "Client says Goodbye".into();

    let mut goodbyes = cfg.goodbye_world(&msg, SendOptions::default())?;

    let mut selection = Selection::new(&mut [&mut hellos, &mut goodbyes]).unwrap();

    loop {
        if let Err(_) = selection.wait(&mut [&mut hellos, &mut goodbyes]) {
            break
        }

        while let Some(hello) = hellos.get_nonblock() {
            match hello.response {
                Ok(msg) => println!("got HelloWorld response from node {}: {}", hello.node.unwrap().id, msg.message),
                Err(e) => println!("HelloWorld response error: err = {:?}", e)
            }
        }

        while let Some(goodbye) = goodbyes.get_nonblock() {
            match goodbye.response {
                Ok(msg) => println!("got GoodbyeWorld response from node {}: {}", goodbye.node.unwrap().id, msg.message),
                Err(e) => println!("GoodbyeWorld response error: err = {:?}", e)
            }
        }
    }

    Ok(())
}