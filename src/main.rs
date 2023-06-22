use circular::Buffer;

use rtcm_rs::{self, prelude::*};
use serde_json;
use tcp_server::TcpServer;
use std::io::{BufRead, BufReader, Read, Write};

mod arguments;
mod coordinate;
mod ntrip_client;
mod tcp_client;
mod tcp_handler;
mod tcp_server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = arguments::parse_arguments().expect("Error parsing arguments");

    let arguments::Arguments {
        input,
        output,
        conv_dir,
    } = arguments;

    let input: Box<dyn BufRead> = setup_input(input)?;
    let output: Box<dyn Write> = setup_output(output)?;

    if let arguments::ConvDir::Forward = conv_dir {
        forward(input, output);
    } else {
        backward(input, output);
    }
    Ok(())
}

fn setup_input(input: arguments::Input) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    match input {
        arguments::Input::StdIn => Ok(Box::new(std::io::stdin().lock())),
        arguments::Input::File { path } => {
            Ok(Box::new(BufReader::new(std::fs::File::open(&path)?)))
        }
        arguments::Input::TcpClient { host, port } => Ok(Box::new(BufReader::new(
            tcp_client::TcpClient::new(host, port, 10),
        ))),
        arguments::Input::NtripClient {
            host,
            port,
            mountpoint,
            username,
            password,
            coordinate,
            nmea_int,
        } => {
            let mut nclient = ntrip_client::NtripClient::new(host, port, 10, mountpoint);
            if let (Some(username), Some(password)) = (username, password) {
                nclient.set_credentials(username, password);
            }
            if let Some(coordinate) = coordinate {
                nclient.set_nmea(coordinate, nmea_int);
            }
            Ok(Box::new(BufReader::new(nclient)))
        }
    }
}

fn setup_output(output: arguments::Output) -> Result<Box<dyn Write>, Box<dyn std::error::Error>> {
    match output {
        arguments::Output::StdOut => Ok(Box::new(std::io::stdout().lock())),
        arguments::Output::File { path } => Ok(Box::new(std::fs::File::create(&path)?)),
        arguments::Output::TcpClient { host, port } => {
            Ok(Box::new(tcp_client::TcpClient::new(host, port, 10)))
        }
        arguments::Output::TcpServer { host, port } => {
            Ok(Box::new(TcpServer::new(host, port)))
        },
    }
}

fn forward(mut rtcm_input: Box<dyn BufRead>, mut json_output: Box<dyn Write>) {
    let mut buffer = Buffer::with_capacity(2 * 1029);
    loop {
        if let Ok(n) = rtcm_input.read(buffer.space()) {
            if n == 0 {
                break;
            }
            buffer.fill(n);
        }
        let mut iter = MsgFrameIter::new(buffer.data());
        for mf in &mut iter {
            if let Ok(json_msg) = serde_json::to_string(&mf.get_message()) {
                let _ = json_output.write_all(json_msg.as_bytes());
                let _ = json_output.write_all("\r\n".as_bytes());
            }
        }
        buffer.consume(iter.consumed());
    }
    let _ = json_output.flush();
}
fn backward(json_input: Box<dyn BufRead>, mut rtcm_output: Box<dyn Write>) {
    let mut msg_builder = MessageBuilder::new();
    for json_msg in json_input.lines() {
        if let Some(msg_data) = json_msg
            .ok()
            .and_then(|msg| serde_json::from_str::<Message>(msg.as_str()).ok())
            .and_then(|msg| msg_builder.build_message(&msg).ok())
        {
            let _ = rtcm_output.write_all(msg_data);
        }
    }
}
