

pub mod configuration {

    use crate::Node;
    use crate::Response;

    use futures::stream::FuturesUnordered;
    use futures::stream::Stream;

    use std::sync::Arc;

    #[derive(Default)]
    pub struct Configuration<NIDT> {
        nodes: Box<[Node<NIDT>]>
    }

    impl<NIDT: Copy> Configuration<NIDT> {
        pub fn new(addrs: &[(String, NIDT)]) -> Self {
            let nodes = addrs.iter().map(|(addr, id)| Node::new(addr.clone(), id.clone())).collect();
            Configuration { nodes: nodes }
        }

        pub fn send<'a, T: Into<Arc<[u8]>>>(&'a self, data: T) -> impl Stream<Item = Response<'a, Vec<u8>, NIDT>>
        {
            let queue = FuturesUnordered::new();

            let data = data.into();

            for node in self.nodes.iter() {
                let send = node.send(data.clone());
                queue.push(send);
            }

            queue
        }
    }
}