

pub mod server {
    use std::net::TcpListener;
    
    use std::io::Read;

    use std::error::Error;

    use std::sync::Arc;
    use std::sync::Mutex;

    use std::thread::spawn;

    use std::io::Write;

    pub struct Server<Handler>
    where 
        Handler: ServerHandler + Send + 'static
    {
        pub handler: Arc<Mutex<Box<Handler>>>
    }

    impl<Handler> Server<Handler>
    where 
        Handler: ServerHandler + Send + 'static
    {
        pub fn new(handler: Box<Handler>) -> Self {
            Server {
                handler: Arc::new(Mutex::new(handler))
            }
        }

        pub fn serve(self, listener: TcpListener) -> Box<dyn Error> {

            loop {
                let (mut socket, _) = match listener.accept() {
                    Ok(s) => s,
                    Err(e) => {
                        return Box::new(e);
                    }
                };

                let handler = self.handler.clone();
                spawn(move || {
                    let mut buf = vec![];
                    
                    loop {
                        let n = match socket.read_to_end(&mut buf) {
                            Ok(0) => {
                                return;
                            }
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("failed to read from socket; err = {:?}", e);
                                return;
                            }
                        };

                        let mut handler = handler.lock().expect("unable to lock mutex");

                        let resp = handler.handle(&buf[0..n]);
                        let resp = match resp {
                            Ok(resp) => resp,
                            Err(e) => {
                                eprintln!("server handler failed; err = {:?}", e);
                                return;
                            }
                        };
                        
                        // Write the data back
                        if let Err(e) = socket.write_all(&resp) {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                });
            }
        }
    }

    pub trait ServerHandler {
        fn handle(&mut self, request: &[u8]) -> Result<Vec<u8>, Box<dyn Error + Send>>;
    }
}