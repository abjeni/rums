

pub mod configuration {

    use crate::Node;
    use crate::node::node::Connection;
    use crate::Response;
    use crate::Responses;

    use std::mem;
    use std::pin::Pin;

    use mio::{event::Event, Interest, Poll, Token};

    use std::io;
    use std::{error, fmt};
    use std::fmt::Display;

    use std::default::Default;

    pub struct SendOptions {
    }

    impl SendOptions {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Default for SendOptions {
        fn default() -> Self {
            Self {

            }
        }
    }

    #[derive(Debug)]
    pub(crate) struct EmptyError {}

    impl EmptyError {
        pub(crate) fn new() -> Self {
            Self {}
        }
    }

    impl error::Error for EmptyError {}

    impl Display for EmptyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "The connection has not written the entire data")
        }
    }

    pub struct ResponseRouter {
        responses: Box<[usize]>,
        pending_responses: usize,
        open_sockets: usize,
    }

    impl ResponseRouter {
        fn new(n: usize) -> Self {
            let responses = vec![0; n].into_boxed_slice();
            Self {
                responses: responses,
                pending_responses: 0,
                open_sockets: n
            }
        }

        fn add(&mut self, i: usize) {
            self.responses[self.pending_responses] = i;
            self.pending_responses += 1;
            self.open_sockets -= 1;
        }

        fn get(&mut self) -> Option<usize> {
            if self.pending_responses > 0 {
                self.pending_responses -= 1;
                let i = self.responses[self.pending_responses];
                Some(i)
            } else {
                None
            }
        }

        pub(crate) fn done(&self) -> bool {
            return self.open_sockets == 0
        }
    }

    pub struct Connections<'a, NIDT> {
        pub(crate) connections: Pin<Box<[(Connection, &'a Node<NIDT>)]>>,

        pub(crate) responses: ResponseRouter
    }

    impl<'a, NIDT> Connections<'a, NIDT> {
        // assume that the poll object has registered all sockets
        fn new(connections: Pin<Box<[(Connection, &'a Node<NIDT>)]>>) -> Self {

            let n = connections.len();

            Self {
                connections: connections,
                responses: ResponseRouter::new(n)
            }
        }
    }

    impl<'a, NIDT> Responses<'a> for Connections<'a, NIDT> {
        type Item = Vec<u8>;
        type NodeID = NIDT;

        fn handle_event(&mut self, poll: &mut Poll, event: &Event) {
            let Token(mem_addr) = event.token();

            let size = mem::size_of::<(Connection, &Node<NIDT>)>();
            let base_addr = Pin::into_inner(self.connections.as_ref()).as_ptr().addr();
            if base_addr > mem_addr {
                return
            }
            let offset = mem_addr - base_addr;
            let remainder = offset % size;
            if remainder != 0 {
                return;
            }
            let i = offset / size;

            if i >= self.connections.len() {
                return;
            }

            if event.is_writable() {
                if let Some(success) = self.connections[i].0.write() {
                    if success {
                        if let Err(e) = poll.registry().reregister(&mut self.connections[i].0.socket, Token(mem_addr), Interest::READABLE) {
                            panic!("reregister error: {:?}", e);
                        }
                    } else {
                        self.responses.add(i);
                        if let Err(e) = poll.registry().deregister(&mut self.connections[i].0.socket) {
                            panic!("deregister error: {:?}", e);
                        }
                    }
                }
            } else if event.is_readable() {
                if self.connections[i].0.read() {
                    self.responses.add(i);
                    if let Err(e) = poll.registry().deregister(&mut self.connections[i].0.socket) {
                        panic!("deregister error: {:?}", e);
                    }
                }
            }
        }

        fn register_poll(&mut self, poll: &Poll) {
            for item in self.connections.iter_mut() {
                let mem_addr = (item as *const (Connection, &Node<NIDT>)).addr();
                if let Err(e) = poll.registry().register(&mut item.0.socket, Token(mem_addr), Interest::WRITABLE) {
                    panic!("register error: {:?}", e);
                }
            }
        }

        fn done(&self) -> bool {
            self.responses.done()
        }

        fn len(&self) -> usize {
            self.connections.len()
        }

        fn get_nonblock<'b>(&mut self) -> Option<Response<'a, Vec<u8>, NIDT>>
        where 
            'a: 'b
        {
            self.responses.get().map(|i| {
                let (connection, node) = &mut self.connections[i];
                connection.response(node)
            })
        }
    }

    #[derive(Default)]
    pub struct Configuration<NIDT> {
        nodes: Box<[Node<NIDT>]>
    }

    impl<NIDT: Clone> Configuration<NIDT> {
        pub fn new(addrs: &[(String, NIDT)]) -> Self {
            let nodes = addrs.iter().map(|(addr, id)| {
                Node::new(addr.clone(), id.clone())
            }).collect();
            Configuration { nodes: nodes }
        }

        pub fn send<'a>(&'a self, data: Box<[u8]>, _options: SendOptions) -> io::Result<Connections<'a, NIDT>>
        {
            let mut connections = Box::<[(Connection, &Node<NIDT>)]>::new_uninit_slice(self.nodes.len());

            for (i, node) in self.nodes.iter().enumerate() {
                let conn = node.start_connection(data.clone())?;
                connections[i].write((conn, node));
            }

            let connections = Box::into_pin(unsafe { connections.assume_init() });

            Ok(Connections::new(connections))
        }
    }
}