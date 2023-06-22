# RTCM-JSON Converter

This is a command-line interface (CLI) application that facilitates the serialization and deserialization of RTCM v.3 to and from JSON format. The application is built on top of the `rtcm-rs` library and thus supports new message formats as they are introduced in the `rtcm-rs` updates.

The application supports several input and output options, including an NTRIP v. 1 client implementation.

## Usage

The application is invoked through the command `rtcm-json` and accepts several options to customize the conversion process.

```
$ rtcm-json --help

Usage: rtcm-json [OPTIONS]

Options:
  -b, --backward                      backward conversion, i.e. from json to binary rtcm
  -i, --stdin-input                   input from standard input [default]
  -f, --file-input <file path>
  -c, --tcp-client-input <<host>:<port>>
                                      input from tcp client connection
  -n, --ntrip-client-input <<host>:<port>>
                                      input from built-in ntrip (v. 1) client. 
                                      (Requires mountpoint argument)
  -m, --mountpoint <ntrip mountpoint>
                                      ntrip caster mountpoint to connect to
  -u, --username <ntrip username>
                                      username if required for connection to ntrip caster
  -p, --password <ntrip password>
                                      password if required for connection to ntrip caster
  -l, --llh <<latitude>,<longitude>,<height>>
                                      coordinate to supply to ntrip caster in 
                                      nmea gga message if required
  -x, --xyz <<x>,<y>,<z>>
                                      coordinate to supply to ntrip caster in 
                                      nmea gga message if required
  -r, --nmea-repeat <nmea repeat interval (s)>
                                      time interval between resend of nmea gga coordinates
  -O, --stdout-output
                                      output to standard output [default]
  -F, --file-output <file path>
                                      output to file
  -C, --tcp-client-output <<host url>:<port>>
                                      output to tcp client connection
  -S, --tcp-server-output <<host>:<port>>
                                      serve output on <host>:<port>
  -h, --help
                                      Print help
```

## Building the Executable

To build the executable, run:

```
cargo build --release
```

This will compile the application and create an executable in the `target/release/` directory.

## License

This project is licensed under the MIT License and Apache-2.0 - see the LICENSE-MIT and LICENSE-APACHE files for details.