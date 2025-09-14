
pub mod node {

    use std::string::String;
    use mio::net::TcpStream;
    use std::net::Shutdown;
    use std::io;
    use std::error::Error;
    use std::mem;
    use std::io::{Read, Write};

    use crate::Response;


    pub(crate) struct Connection {
        pub(crate) response: io::Result<Vec<u8>>,
        pub(crate) socket: Box<TcpStream>,
        bytes_written: usize,
        bytes_read: usize,
        buf: Vec<u8>,
        data: Box<[u8]>
    }

    impl Connection {
        pub(crate) fn new(socket: TcpStream, data: Box<[u8]>) -> Self {
            Connection {
                response: Err(io::Error::new(io::ErrorKind::InProgress, Box::<dyn Error + Send + Sync>::from("response is pending"))),
                bytes_written: 0,
                bytes_read: 0,
                buf: vec![0; 4096],
                socket: Box::new(socket),
                data: data
            }
        }

        // data should be part of Connection, it needs to be the same for every call
        // returns true if it is successful, and false when unsuccessful
        pub(crate) fn write(&mut self) -> Option<bool> {
            loop {
                if self.data.len() == self.bytes_written {
                    if let Err(e) = self.socket.shutdown(Shutdown::Write) {
                        panic!("reregister error: {:?}", e);
                    };
                    return Some(true);
                }

                let n = match self.socket.write(&self.data[self.bytes_written..]) {
                    Ok(n) => n,
                    Err(e) if e.kind() == io::ErrorKind::NotConnected => return None,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => return None,
                    Err(e) => {
                        self.response = Err(e);
                        return Some(false);
                    }
                };
                self.bytes_written += n;
            }
        }

        // non-blocking function: returns true when complete
        pub(crate) fn read(&mut self) -> bool {

            loop {
                match self.socket.read(&mut self.buf[self.bytes_read..]) {
                    Ok(0) => {
                        break;
                    }
                    Ok(n) => {
                        self.bytes_read += n;
                        if self.bytes_read == self.buf.len() {
                            self.buf.resize(self.buf.len() + 4096, 0);
                        }
                    }
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) if e.kind() == io::ErrorKind::NotConnected => return false,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => return false,
                    Err(e) => {
                        self.response = Err(e);
                        return true;
                    }
                }
            }
            self.buf.truncate(self.bytes_read);
            self.response = Ok(mem::replace(&mut self.buf, vec![]));
            return true;
        }

        pub(crate) fn response<'a, NIDT>(&mut self, node: &'a Node<NIDT>) -> Response<'a, Vec<u8>, NIDT> {
            Response{
                response: mem::replace(&mut self.response, Err(io::Error::new(io::ErrorKind::InProgress, Box::<dyn Error + Send + Sync>::from("response is pending")))),
                node: Some(node)
            }
        }
    }

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

        pub(crate) fn start_connection<'a>(&'a self, data: Box<[u8]>) -> io::Result<Connection> {
            let socket = TcpStream::connect(self.addr.parse().expect("addr invalid"))?;

            let conn = Connection::new(socket, data);
            Ok(conn)
        }
    }
}