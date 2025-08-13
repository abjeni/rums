


pub mod route {
    
    use std::error::Error;

    use std::collections::HashMap;

    use crate::ServerHandler;
    
    pub fn get_route(data: &[u8]) -> (&[u8], &str) {

        let mut i = 0;
        for c in data {
            if *c == 0 {
                break;
            }
            i += 1;
        }

        if i == data.len() {
            return (data, "")
        }

        let route = match str::from_utf8(&data[0..i]) {
            Ok(route) => route,
            Err(_) => return (data, "")
        };

        (&data[i+1..], route)
    }
    
    pub fn add_route(data: Vec<u8>, route: &str) -> Vec<u8> {
        [route.into(), data].join(&0)
    }

    pub struct RouteHandler {
        pub subhandlers: HashMap<String, Box<Self>>,
        pub handlers: HashMap<String, Box<dyn ServerHandler + Send>>
    }

    impl RouteHandler {
        pub fn new() -> Self {
            Self {
                handlers: HashMap::new(),
                subhandlers: HashMap::new()
            }
        }

        pub fn add_route(&mut self, route: &str, handler: Box<dyn ServerHandler + Send>) -> &mut Box<dyn ServerHandler + Send> {
            self.handlers.entry(String::from(route)).or_insert(handler)
        }

        pub fn get_route(&mut self, route: &str) -> Option<&mut Box<dyn ServerHandler + Send>> {
            self.handlers.get_mut(route)
        }

        pub fn sub_route(&mut self, route: &str) -> &mut Box<Self> {
            self.subhandlers.entry(String::from(route)).or_insert(Box::new(Self::new()))
        }
    }

    impl ServerHandler for RouteHandler {
        fn handle(&mut self, request: &[u8]) -> Result<Vec<u8>, Box<dyn Error + Send>> {

            let (request, route) = get_route(request);

            if self.handlers.contains_key(route) {
                return self.handlers.get_mut(route).unwrap().handle(request);
            }

            match self.subhandlers.contains_key(route) {
                true => self.subhandlers.get_mut(route).unwrap().handle(request),
                false => Err(Box::<dyn Error + Send + Sync>::from(format!("unhandled route: {}", route)))
            }
        }
    }

    impl Clone for RouteHandler {
        fn clone(&self) -> Self {
            RouteHandler::new()
        }
    }
}