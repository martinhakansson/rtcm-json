use crate::coordinate::Coordinate;
use crate::tcp_handler::TcpHandler;
use base64::Engine as _;
use std::io::{Read, Write};
use std::time::{Duration, Instant};

pub struct NtripClient {
    tcp_handler: TcpHandler,
    mountpoint: String,
    credentials: Option<Credentials>,
    nmea_freq: Option<Duration>,
    nmea_coord: Option<Coordinate>,
    latest_nmea_write: Option<Instant>,
}

struct Credentials {
    username: String,
    password: String,
}

impl NtripClient {
    pub fn new(addr: String, port: u16, reconnect: u64, mountpoint: String) -> Self {
        NtripClient {
            tcp_handler: TcpHandler::new(addr, port, Duration::from_secs(reconnect)),
            mountpoint,
            credentials: None,
            nmea_freq: None,
            nmea_coord: None,
            latest_nmea_write: None,
        }
    }
    pub fn set_credentials(&mut self, username: String, password: String) {
        self.credentials = Some(Credentials { username, password });
    }
    pub fn set_nmea(&mut self, nmea_coord: Coordinate, nmea_freq: Option<u64>) {
        self.nmea_coord = Some(nmea_coord);
        self.nmea_freq = nmea_freq.map(|nf| Duration::from_secs(nf));
    }
}

impl Read for NtripClient {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let stream = match self.tcp_handler.get_stream_reconnect() {
            crate::tcp_handler::Connection::ExistingConnection(stream) => stream,
            crate::tcp_handler::Connection::NewConnection(stream) => {
                //initialize connection
                stream.write_all(
                    format!(
                        "GET /{} HTTP/1.0\r\nUser-Agent: NTRIP rtcm-json/0.1\r\nAccept: */*\r\n{}\r\n",
                        &self.mountpoint,
                        if let Some(cr) = self.credentials.as_ref() {
                            format!("Authorization: Basic {}\r\n", encode_credentials(cr))
                        } else {
                            "".into()
                        }
                    )
                    .as_bytes(),
                )?;                
                //verify response
                let mut resp_buf: [u8; 12] = [0; 12];
                let mut written: usize = 0;
                while written < resp_buf.len() {
                    let n = stream.read(&mut resp_buf[written..])?;
                    written += n;
                    if &"ICY 200 OK\r\n".as_bytes()[..written] != &resp_buf[..written] {
                        return Err(std::io::ErrorKind::PermissionDenied.into());
                    }                    
                }
                if "ICY 200 OK\r\n".as_bytes() != &resp_buf {
                    return Err(std::io::ErrorKind::PermissionDenied.into());
                }
                //send nmea gga
                if let Some(coord) = self.nmea_coord.as_ref() {
                    coord.write_to_stream(stream)?;
                    self.latest_nmea_write = Some(Instant::now());
                }                
                stream
            }
        };
        //check if it's time to send nmea gga position message
        if let (Some(freq), Some(coord), Some(latest_write)) = (
            self.nmea_freq.as_ref(),
            self.nmea_coord.as_ref(),
            self.latest_nmea_write.as_ref(),
        ) {
            if Instant::now() - *latest_write > *freq {
                coord.write_to_stream(stream)?;
                self.latest_nmea_write = Some(Instant::now());
            }
        }

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

fn encode_credentials(credentials: &Credentials) -> String {
    base64::engine::general_purpose::STANDARD
        .encode(format!("{}:{}", credentials.username, credentials.password))
}
