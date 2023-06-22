use crate::tcp_handler::TcpHandler;
use std::io::{Read, Write};
use std::time::Duration;

pub struct TcpClient {
    // Change to use TcpReconnect
    tcp_handler: TcpHandler,
}

impl TcpClient {
    pub fn new(addr: String, port: u16, reconnect: u64) -> Self {
        TcpClient {
            tcp_handler: TcpHandler::new(addr, port, Duration::from_secs(reconnect)),
        }
    }
}

impl Read for TcpClient {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let stream = match self.tcp_handler.get_stream_reconnect() {
            crate::tcp_handler::Connection::ExistingConnection(stream) => stream,
            crate::tcp_handler::Connection::NewConnection(stream) => stream,
        };
        match stream.read(&mut *buf) {
            Ok(v) => {
                if v == 0 {
                    self.tcp_handler.discard_stream();
                    Err(std::io::ErrorKind::NotConnected.into())
                } else {
                    Ok(v)
                }
            }
            Err(v) => {
                self.tcp_handler.discard_stream();
                Err(v)
            }
        }
    }
}

impl Write for TcpClient {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let stream = match self.tcp_handler.get_stream_reconnect() {
            crate::tcp_handler::Connection::ExistingConnection(stream) => stream,
            crate::tcp_handler::Connection::NewConnection(stream) => stream,
        };

        match stream.write(buf) {
            Ok(v) => {
                if v == 0 {
                    self.tcp_handler.discard_stream();
                    Err(std::io::ErrorKind::NotConnected.into())
                } else {
                    Ok(v)
                }
            }
            Err(v) => {
                self.tcp_handler.discard_stream();
                Err(v)
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(stream) = self.tcp_handler.get_stream() {
            let flush_res = stream.flush();
            if flush_res.is_err() {
                self.tcp_handler.discard_stream();
            }
            flush_res
        } else {
            Err(std::io::ErrorKind::NotConnected.into())
        }
    }
}
