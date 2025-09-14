
pub mod response {

    use crate::Node;
    use crate::configuration::configuration::EmptyError;
    use crate::configuration::configuration::Connections;
    
    use std::io;

    use prost::Message;

    use std::marker::PhantomData;

    use mio::{event::Event, Events, Poll};

    pub struct Response<'a, T, NIDT> {
        pub response: Result<T, io::Error>,
        pub node: Option<&'a Node<NIDT>>
    }

    pub trait Responses<'a> {
        type Item;
        type NodeID;
        fn handle_event(&mut self, poll: &mut Poll, event: &Event);
        fn get_nonblock<'b>(&mut self) -> Option<Response<'b, Self::Item, Self::NodeID>> where 'a: 'b;
        fn register_poll(&mut self, poll: &Poll);
        fn done(&self) -> bool;
        fn len(&self) -> usize;
        fn iter_block(&'a mut self) -> ResponsesBlocking<'a, Self> {
            ResponsesBlocking::new::<Self::NodeID>(self)
        }
        fn iter_nonblock(&'a mut self) -> ResponsesNonblock<'a, Self> {
            ResponsesNonblock::new::<Self::NodeID>(self)
        }
    }

    pub struct ResponsesMap<'a, T, NIDT> {
        responses: Connections<'a, NIDT>,
        response_type: PhantomData<&'a T>
    }

    impl<'a, T, NIDT> ResponsesMap<'a, T, NIDT> {
        pub fn new(responses: Connections<'a, NIDT>) -> Self {
            ResponsesMap{
                responses: responses,
                response_type: PhantomData
            }
        }
    }
    
    impl<'a, T: Message + Default, NIDT> Responses<'a> for ResponsesMap<'a, T, NIDT> {
        type Item = T;
        type NodeID = NIDT;
        fn handle_event(&mut self, poll: &mut Poll, event: &Event) {
            self.responses.handle_event(poll, event)
        }
        fn get_nonblock<'b>(&mut self) -> Option<Response<'b, Self::Item, Self::NodeID>> where 'a: 'b {

            let res = self.responses.get_nonblock();

            res.map(|res|
                Response{
                    response: match res.response {
                        Ok(resp) => T::decode(&resp as &[u8]).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
                        Err(e) => Err(e)
                    },
                    node: res.node
                }
            )
        }
        fn register_poll(&mut self, poll: &Poll) {
            self.responses.register_poll(poll)
        }
        fn done(&self) -> bool {
            self.responses.done()
        }
        fn len(&self) -> usize {
            self.responses.len()
        }
    }

    pub struct ResponsesNonblock<'a, R: Responses<'a> + ?Sized> {
        responses: &'a mut R
    }
    
    impl<'a, R: Responses<'a> + ?Sized> ResponsesNonblock<'a, R> {
        pub(crate) fn new<NIDT>(responses: &'a mut R) -> Self {
            Self {
                responses: responses
            }
        }
    }

    impl<'a, R: Responses<'a> + ?Sized> Iterator for ResponsesNonblock<'a, R> {
        type Item = Response<'a, R::Item, R::NodeID>;

        fn next(&mut self) -> Option<Response<'a, R::Item, R::NodeID>>
        {
            self.responses.get_nonblock()
        }
    }

    pub struct ResponsesBlocking<'a, R: Responses<'a> + ?Sized> {
        responses: &'a mut R,
        poll: Poll,
        events: Events
    }

    impl<'a, R: Responses<'a> + ?Sized> ResponsesBlocking<'a, R> {
        pub(crate) fn new<NIDT>(responses: &'a mut R) -> Self {

            let n = responses.len();

            let events = Events::with_capacity(n);

            let mut poll = match Poll::new() {
                Ok(poll) => poll,
                Err(e) => {
                    panic!("Poll::new() failed: {:?}", e);
                }
            };

            responses.register_poll(&mut poll);

            Self {
                responses: responses,
                poll: poll,
                events: events
            }
        }

        // blocks but may not return a response
        fn poll<'b>(&mut self) -> io::Result<()>
        where 
            'a: 'b
        {
            if self.responses.done() {
                return Err(io::Error::new(io::ErrorKind::NotFound, EmptyError::new()));
            }

            let mut events = &mut self.events;

            if let Err(e) = self.poll.poll(&mut events, None) {
                return Err(e);
            }

            for event in events.iter() {
                self.responses.handle_event(&mut self.poll, event);
            }
            Ok(())
        }

        // blocks until it returns a response from a node
        fn get_block<'b>(&mut self) -> Response<'b, R::Item, R::NodeID>
        where 
            'a: 'b
        {
            loop {
                if let Some(res) = self.responses.get_nonblock() {
                    return res;
                }

                if let Err(e) = self.poll() {
                    return Response{
                        response: Err(e),
                        node: None
                    };
                }
            }
        }
    }

    impl<'a, R: Responses<'a> + ?Sized> Iterator for ResponsesBlocking<'a, R> {
        type Item = Response<'a, R::Item, R::NodeID>;

        fn next(&mut self) -> Option<Response<'a, R::Item, R::NodeID>>
        {
            let res = self.get_block();

            if let Err(ref e) = res.response {
                if e.kind() == io::ErrorKind::NotFound {
                    return None
                }
            }
            Some(res)
        }
    }

    pub struct Selection {
        poll: Poll,
        events: Events
    }

    impl Selection {
        pub fn new<'a, R: Responses<'a> + ?Sized>(responses: &mut [&mut R]) -> io::Result<Self> {
            let n = responses.iter().map(|c| c.len()).sum();

            let events = Events::with_capacity(n);

            let poll = Poll::new()?;

            for r in responses.iter_mut() {
                r.register_poll(&poll);
            }

            Ok(Self {
                poll: poll,
                events: events
            })
        }

        pub fn wait<'a, R: Responses<'a> + ?Sized>(&mut self, responses: &mut [&mut R]) -> io::Result<()> {
            if responses.iter().all(|c| c.done()) {
                return Err(io::Error::new(io::ErrorKind::NotFound, EmptyError::new()));
            }

            let mut events = &mut self.events;
            if let Err(e) = self.poll.poll(&mut events, None) {
                return Err(e);
            }

            for c in responses.iter_mut() {
                if c.done() {
                    continue;
                }

                for event in events.iter() {
                    c.handle_event(&mut self.poll, event);
                }
            }
            Ok(())
        }
    }
}