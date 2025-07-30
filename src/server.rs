

pub mod server {
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::runtime::Builder;

    use std::error::Error;

    use std::sync::Arc;
    use tokio::sync::Mutex;

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

        pub async fn serve(self, listener: TcpListener) -> Box<dyn Error> {
            
            let runtime = Builder::new_multi_thread().build().unwrap();

            loop {
                let (mut socket, _) = match listener.accept().await {
                    Ok(s) => s,
                    Err(e) => {
                        runtime.shutdown_background();
                        return Box::new(e);
                    }
                };

                let handler = self.handler.clone();
                runtime.spawn(async move {
                    let mut buf = vec![];
                    
                    loop {
                        let n = match socket.read_buf(&mut buf).await {
                            Ok(0) => return,
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("failed to read from socket; err = {:?}", e);
                                return;
                            }
                        };

                        let handler = handler.lock().await;

                        let resp = handler.handle(&buf[0..n]);
                        let resp = match resp {
                            Ok(resp) => resp,
                            Err(e) => {
                                eprintln!("server handler failed; err = {:?}", e);
                                return;
                            }
                        };

                        // Write the data back
                        if let Err(e) = socket.write_all(&resp).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                });
            }
        }
    }

    pub trait ServerHandler {
        fn handle(&'_ self, request: &[u8]) -> Result<Vec<u8>, Box<dyn Error + Send>>;
    }

    impl<'a> ServerHandler for fn(request: &[u8]) -> Vec<u8> {
        fn handle(&'_ self, request: &[u8]) -> Result<Vec<u8>, Box<dyn Error + Send>> {
            let response = (self)(request);

            Ok(response)
        }
    }
}