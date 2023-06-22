use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver};
use std::io::Write;


pub struct TcpServer {
    new_connections:Receiver<TcpStream>,
    connections:Vec<TcpStream>,
}

impl TcpServer {
    pub fn new(host: String, port: u16) -> Self {
        let listener =
            TcpListener::bind(format!("{}:{}", host, port)).expect("Could not bind to port");
        let (sender,new_connections) = channel();
        let _ = std::thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    stream.set_nonblocking(true).expect("could not set non-blocking mode on tcp connection");
                    if sender.send(stream).is_err() {
                        break;
                    }
                }                
            }
        });
        TcpServer {
            new_connections,
            connections: Vec::new(),
        }
    }
    fn add_new_connection(&mut self) {
        for nc in self.new_connections.iter() {
            self.connections.push(nc);
        }
    }
}

impl Write for TcpServer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.add_new_connection();
        self.connections.retain_mut(|c| {
            match c.write(buf) {
                Ok(_) => true,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        true
                    } else {
                        false
                    }
                },
            }
        });
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.add_new_connection();
        self.connections.retain_mut(|c| {
            match c.flush() {
                Ok(_) => true,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        true
                    } else {
                        false
                    }
                },
            }
        });
        Ok(())
    }
}


