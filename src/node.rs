

pub mod node {

    use std::string::String;

    use tokio::net::TcpStream;

    use tokio::io::AsyncWriteExt;
    use tokio::io::AsyncReadExt;
    
    use std::error::Error;

    pub struct Node {
        addr: String
    }

    impl Node {
        pub fn new(addr: String) -> Self {
            Node {
                addr: addr
            }
        }

        pub async fn send(&self, data: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
            let mut stream = TcpStream::connect(self.addr.clone()).await?;

            stream.write_all(&data).await?;

            let mut buf = vec![];
            
            // consider returning an error on a 0 byte response
            let n = match stream.read_buf(&mut buf).await {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return Err(Box::new(e));
                }
            };

            assert_eq!(buf.len(), n);

            Ok(buf)
        }
    }
}