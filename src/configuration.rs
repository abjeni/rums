

pub mod configuration {

    use crate::Node;

    use futures::stream::FuturesUnordered;
    use futures::stream::Stream;
    
    use std::error::Error;

    #[derive(Default)]
    pub struct Configuration {
        nodes: Vec<Node>
    }

    impl Configuration {
        pub fn new(addrs: &[String]) -> Self {
            let mut nodes = vec![];

            for addr in addrs.iter() {
                nodes.push(Node::new(addr.clone()))
            }

            Configuration { nodes: nodes }
        }

        pub fn send<'a>(&'a self, data: &[u8]) -> impl Stream<Item = Result<Vec<u8>, Box<dyn Error>>> + Unpin + use<'a> {
            let queue = FuturesUnordered::new();

            for node in self.nodes.iter() {
                let send = node.send(Vec::from(data));
                queue.push(send);
            }

            queue
        }
    }
}