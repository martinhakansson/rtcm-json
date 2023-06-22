use std::net::TcpStream;
use std::time::{Duration, Instant};

pub struct TcpHandler {
    addr: String,
    port: u16,
    tcp_stream: Option<TcpStream>,
    reconnect: Duration,
    last_connect: Instant,
}

pub enum Connection<'a> {
    ExistingConnection(&'a mut TcpStream),
    NewConnection(&'a mut TcpStream),
}

impl TcpHandler {
    pub fn new(addr: String, port: u16, reconnect: Duration) -> Self {
        TcpHandler {
            addr,
            port,
            tcp_stream: None,
            reconnect,
            last_connect: Instant::now() - reconnect,
        }
    }
    pub fn get_stream_reconnect(&mut self) -> Connection<'_> {
        if self.tcp_stream.is_some() {
            return Connection::ExistingConnection(self.tcp_stream.as_mut().unwrap());
        }
        while self.tcp_stream.is_none() {
            let sleep_time = self
                .reconnect
                .saturating_sub(Instant::now() - self.last_connect);
            if !sleep_time.is_zero() {
                std::thread::sleep(sleep_time);
            }
            let _ = self.connect();
        }
        Connection::NewConnection(self.tcp_stream.as_mut().unwrap())
    }
    pub fn get_stream(&mut self) -> Option<&mut TcpStream> {
        self.tcp_stream.as_mut()
    }
    pub fn discard_stream(&mut self) {
        self.tcp_stream = None;
    }
    fn connect(&mut self) -> std::io::Result<()> {
        self.last_connect = Instant::now();
        self.tcp_stream = Some(TcpStream::connect(format!("{}:{}", self.addr, self.port))?);
        Ok(())
    }
}
