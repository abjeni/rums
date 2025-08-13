

pub mod node {

    use std::string::String;

    use tokio::net::TcpStream;

    use tokio::io::AsyncWriteExt;
    use tokio::io::AsyncReadExt;
    
    use crate::Response;
    
    use std::error::Error;

    use std::sync::Arc;

    pub struct Node<NIDT> {
        pub addr: String,
        pub id: NIDT
    }

    impl<NIDT> Node<NIDT> {
        pub fn new(addr: String, id: NIDT) -> Self {
            Node {
                addr: addr,
                id: id
            }
        }

        pub async fn send(&self, data: Arc<[u8]>) -> Response<Vec<u8>, NIDT> {
            let mut stream = match TcpStream::connect(self.addr.clone()).await {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("failed connect to socket; err = {:?}", e);
                    return Response {
                        response: Err(Box::new(e)),
                        node: self
                    }
                }
            };

            if let Err(e) = stream.write_all(&data).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return Response {
                    response: Err(Box::new(e)),
                    node: self
                }
            }

            if let Err(_) = stream.shutdown().await {
                return Response {
                    response: Err(Box::<dyn Error>::from("shutdown error")),
                    node: self
                }
            }

            let mut buf = vec![];
            
            // consider returning an error on a 0 byte response
            if let Err(e) = stream.read_buf(&mut buf).await {
                eprintln!("failed to read from socket; err = {:?}", e);
                return Response {
                    response: Err(Box::new(e)),
                    node: self
                }
            }

            Response {
                response: Ok(buf),
                node: self
            }
        }
    }
}