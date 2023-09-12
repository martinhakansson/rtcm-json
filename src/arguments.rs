use crate::coordinate::Coordinate;
use clap::{Arg, ArgGroup, Command};

pub struct Arguments {
    pub input: Input,
    pub output: Output,
    pub conv_dir: ConvDir,
    pub pretty_print:bool,
}

pub enum Input {
    StdIn,
    File {
        path: String,
    },
    TcpClient {
        host: String,
        port: u16,
    },
    NtripClient {
        host: String,
        port: u16,
        mountpoint: String,
        username: Option<String>,
        password: Option<String>,
        coordinate: Option<Coordinate>,
        nmea_int: Option<u64>,
    },
}

pub enum Output {
    StdOut,
    File { path: String },
    TcpClient { host: String, port: u16 },
    TcpServer { host: String, port: u16 },
}

pub enum ConvDir {
    Forward,
    Backward,
}

#[derive(Debug)]
pub struct CoordinateParseError;

impl std::fmt::Display for CoordinateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("error parsing coordinate")
    }
}
impl std::error::Error for CoordinateParseError {}

const REVERSE_ID: &'static str = "reverse";
const STDIN_INPUT_ID: &'static str = "stdin-input";
const FILE_INPUT_ID: &'static str = "file-input";
const TCP_CLIENT_INPUT_ID: &'static str = "tcp-client-input";
const NTRIP_CLIENT_INPUT_ID: &'static str = "ntrip-client-input";
const MOUNTPOINT_ID: &'static str = "mountpoint";
const USERNAME_ID: &'static str = "username";
const PASSWORD_ID: &'static str = "password";
const LLH_COORDINATE_ID: &'static str = "llh-coordinate";
const XYZ_COORDINATE_ID: &'static str = "xyz-coordinate";
const NMEA_REPEAT_INTERVAL_ID: &'static str = "nmea-repeat-interval";
const STDOUT_OUTPUT_ID: &'static str = "stdout-ouput";
const FILE_OUTPUT_ID: &'static str = "file-output";
const TCP_CLIENT_OUTPUT_ID: &'static str = "tcp-client-output";
const TCP_SERVER_OUTPUT_ID: &'static str = "tcp-server-output";
const PRETTY_PRINT_ID: &'static str = "pretty-print";
const INPUT_GROUP_ID: &'static str = "input-group";
const OUTPUT_GROUP_ID: &'static str = "output-group";
const COORDINATE_GROUP_ID: &'static str = "coordinate-group";

pub fn parse_arguments() -> Result<Arguments, ()> {
    let matches = Command::new("rtcm-json")
        .version(version!())
        .about("JSON serialization/deserialization of RTCM v. 3")
        .arg(
            Arg::new(REVERSE_ID)
                .short('b')
                .long("backward")
                .help("backward conversion, i.e. from json (ndjson) to binary rtcm")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new(STDIN_INPUT_ID)
                .short('i')
                .long("stdin-input")
                .help("input from standard input [default]")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new(FILE_INPUT_ID)
                .short('f')
                .long("file-input")
                .value_name("file path")
                .help("input from file")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new(TCP_CLIENT_INPUT_ID)
                .short('c')
                .long("tcp-client-input")
                .value_name("<host>:<port>")
                .help("input from tcp client connection")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new(NTRIP_CLIENT_INPUT_ID)
                .short('n')
                .long("ntrip-client-input")
                .value_name("<host>:<port>")
                .help("input from built-in Ntrip (v. 1) client. \n(Requires mountpoint argument)")
                .action(clap::ArgAction::Set)
                .requires(MOUNTPOINT_ID),
        )
        .arg(
            Arg::new(MOUNTPOINT_ID)
                .short('m')
                .long("mountpoint")
                .value_name("Ntrip mountpoint")
                .help("Ntrip caster mountpoint to connect to")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new(USERNAME_ID)
                .short('u')
                .long("username")
                .value_name("Ntrip username")
                .help("username if required for connection to Ntrip caster")
                .action(clap::ArgAction::Set)
                .requires(PASSWORD_ID),
        )
        .arg(
            Arg::new(PASSWORD_ID)
                .short('p')
                .long("password")
                .value_name("Ntrip password")
                .help("password if required for connection to Ntrip caster")
                .action(clap::ArgAction::Set)
                .requires(USERNAME_ID),
        )
        .arg(
            Arg::new(LLH_COORDINATE_ID)
                .short('l')
                .long("llh")
                .value_name("<latitude>,<longitude>,<height>>")
                .help("coordinate to supply to Ntrip caster in \nnmea gga message if required")
                .next_line_help(true)
                .action(clap::ArgAction::Set)
                .value_parser(|v: &str| -> Result<Coordinate, CoordinateParseError> {
                    let mut v_iter = v.split(',');
                    let lat =
                        if let Some(v) = v_iter.next().and_then(|v| v.trim().parse::<f64>().ok()) {
                            v
                        } else {
                            return Err(CoordinateParseError);
                        };
                    let lon =
                        if let Some(v) = v_iter.next().and_then(|v| v.trim().parse::<f64>().ok()) {
                            v
                        } else {
                            return Err(CoordinateParseError);
                        };
                    let height =
                        if let Some(v) = v_iter.next().and_then(|v| v.trim().parse::<f64>().ok()) {
                            v
                        } else {
                            return Err(CoordinateParseError);
                        };

                    Ok(Coordinate::from_llh(lat, lon, height))
                }),
        )
        .arg(
            Arg::new(XYZ_COORDINATE_ID)
                .short('x')
                .long("xyz")
                .value_name("<x>,<y>,<z>>")
                .help("coordinate to supply to Ntrip caster in \nnmea gga message if required")
                .next_line_help(true)
                .action(clap::ArgAction::Set)
                .value_parser(|v: &str| -> Result<Coordinate, CoordinateParseError> {
                    let mut v_iter = v.split(',');
                    let x =
                        if let Some(v) = v_iter.next().and_then(|v| v.trim().parse::<f64>().ok()) {
                            v
                        } else {
                            return Err(CoordinateParseError);
                        };
                    let y =
                        if let Some(v) = v_iter.next().and_then(|v| v.trim().parse::<f64>().ok()) {
                            v
                        } else {
                            return Err(CoordinateParseError);
                        };
                    let z =
                        if let Some(v) = v_iter.next().and_then(|v| v.trim().parse::<f64>().ok()) {
                            v
                        } else {
                            return Err(CoordinateParseError);
                        };

                    Ok(Coordinate::from_xyz(x, y, z))
                }),
        )
        .arg(
            Arg::new(NMEA_REPEAT_INTERVAL_ID)
                .short('r')
                .long("nmea-repeat")
                .value_name("NMEA repeat interval (s)")
                .help("time interval between resend of NMEA GGA coordinates")
                .action(clap::ArgAction::Set)
                .value_parser(clap::value_parser!(u64)),
        )
        .arg(
            Arg::new(STDOUT_OUTPUT_ID)
                .short('O')
                .long("stdout-output")
                .help("output to standard output [default]")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new(FILE_OUTPUT_ID)
                .short('F')
                .long("file-output")
                .value_name("file path")
                .help("output to file")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new(TCP_CLIENT_OUTPUT_ID)
                .short('C')
                .long("tcp-client-output")
                .value_name("<host>:<port>")
                .help("output to tcp client connection")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new(TCP_SERVER_OUTPUT_ID)
                .short('S')
                .long("tcp-server-output")
                .value_name("<host>:<port>")
                .help("serve output on <host>:<port>")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new(PRETTY_PRINT_ID)
                .short('P')
                .long("pretty-print")
                .value_name("pretty print")
                .help("pretty print json output (this format is not valid for backward conversion)")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new(INPUT_GROUP_ID)
                .arg(STDIN_INPUT_ID)
                .arg(FILE_INPUT_ID)
                .arg(TCP_CLIENT_INPUT_ID)
                .arg(NTRIP_CLIENT_INPUT_ID),
        )
        .group(
            ArgGroup::new(OUTPUT_GROUP_ID)
                .arg(STDOUT_OUTPUT_ID)
                .arg(FILE_OUTPUT_ID)
                .arg(TCP_CLIENT_OUTPUT_ID)
                .arg(TCP_SERVER_OUTPUT_ID),
        )
        .group(
            ArgGroup::new(COORDINATE_GROUP_ID)
                .arg(LLH_COORDINATE_ID)
                .arg(XYZ_COORDINATE_ID),
        )
        .get_matches();

    Ok(Arguments {
        input: match &matches {
            matches if matches.contains_id(FILE_INPUT_ID) => Input::File {
                path: matches.get_one::<String>(FILE_INPUT_ID).unwrap().clone(),
            },
            matches if matches.contains_id(TCP_CLIENT_INPUT_ID) => {
                let (host, port) =
                    parse_host_port(&matches.get_one::<String>(TCP_CLIENT_INPUT_ID).unwrap())
                        .expect("host and port incorrectly specified");

                Input::TcpClient { host, port }
            }
            matches if matches.contains_id(NTRIP_CLIENT_INPUT_ID) => {
                let (host, port) =
                    parse_host_port(&matches.get_one::<String>(NTRIP_CLIENT_INPUT_ID).unwrap())
                        .expect("host and port incorrectly specified");
                let mountpoint = matches.get_one::<String>(MOUNTPOINT_ID).unwrap().clone();
                let username = matches.get_one::<String>(USERNAME_ID).cloned();
                let password = matches.get_one::<String>(PASSWORD_ID).cloned();
                let coordinate = matches
                    .get_one::<Coordinate>(LLH_COORDINATE_ID)
                    .or(matches.get_one::<Coordinate>(XYZ_COORDINATE_ID))
                    .cloned();
                let nmea_int = matches.get_one::<u64>(NMEA_REPEAT_INTERVAL_ID).map(|r| *r);

                Input::NtripClient {
                    host,
                    port,
                    mountpoint,
                    username,
                    password,
                    coordinate,
                    nmea_int,
                }
            }
            _ => Input::StdIn,
        },
        output: match &matches {
            matches if matches.contains_id(FILE_OUTPUT_ID) => Output::File {
                path: matches.get_one::<String>(FILE_OUTPUT_ID).unwrap().clone(),
            },
            matches if matches.contains_id(TCP_CLIENT_OUTPUT_ID) => {
                let (host, port) =
                    parse_host_port(&matches.get_one::<String>(TCP_CLIENT_OUTPUT_ID).unwrap())
                        .expect("host and port incorrectly specified");

                Output::TcpClient { host, port }
            }
            matches if matches.contains_id(TCP_SERVER_OUTPUT_ID) => {
                let (host, port) =
                    parse_host_port(&matches.get_one::<String>(TCP_SERVER_OUTPUT_ID).unwrap())
                        .expect("host and port incorrectly specified");

                Output::TcpServer { host, port }
            }
            _ => Output::StdOut,
        },
        conv_dir: if *matches.get_one::<bool>(REVERSE_ID).unwrap() {
            ConvDir::Backward
        } else {
            ConvDir::Forward
        },
        pretty_print: *matches.get_one::<bool>(PRETTY_PRINT_ID).unwrap(),
    })
}

fn parse_host_port(addr: &str) -> Result<(String, u16), ()> {
    let mut v_iter = addr.split(':');
    let host = if let Some(v) = v_iter.next() {
        v
    } else {
        return Err(());
    };
    let port = if let Some(v) = v_iter.next().and_then(|v| v.trim().parse::<u16>().ok()) {
        v
    } else {
        return Err(());
    };
    Ok((host.to_string(), port))
}
