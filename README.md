# RTCM-JSON Converter

This is a command-line interface (CLI) application that facilitates the serialization and deserialization of RTCM v.3 to and from JSON format. The application is built on top of the `rtcm-rs` library and thus supports new message formats as they are introduced in the `rtcm-rs` updates.

The application supports several input and output options, including an NTRIP v. 1 client implementation.

## Pre-Compiled Binaries

[Pre-compiled binaries](https://github.com/martinhakansson/rtcm-json/releases) are provided for various platform to allow for quick and easy setup without the need for building the application from source.

## Usage

The application is invoked through the command `rtcm-json` and accepts several options to customize the conversion process.

```
$ rtcm-json --help

Usage: rtcm-json [OPTIONS]

Options:
  -b, --backward
          backward conversion, i.e. from json (ndjson) to binary rtcm
  -i, --stdin-input
          input from standard input [default]
  -f, --file-input <file path>
          input from file
  -c, --tcp-client-input <<host>:<port>>
          input from tcp client connection
  -n, --ntrip-client-input <<host>:<port>>
          input from built-in Ntrip (v. 1) client. 
          (Requires mountpoint argument)
  -m, --mountpoint <Ntrip mountpoint>
          Ntrip caster mountpoint to connect to
  -u, --username <Ntrip username>
          username if required for connection to Ntrip caster
  -p, --password <Ntrip password>
          password if required for connection to Ntrip caster
  -l, --llh <<latitude>,<longitude>,<height>>>
          coordinate to supply to Ntrip caster in 
          nmea gga message if required
  -x, --xyz <<x>,<y>,<z>>>
          coordinate to supply to Ntrip caster in 
          nmea gga message if required
  -r, --nmea-repeat <NMEA repeat interval (s)>
          time interval between resend of NMEA GGA coordinates
  -O, --stdout-output
          output to standard output [default]
  -F, --file-output <file path>
          output to file
  -C, --tcp-client-output <<host>:<port>>
          output to tcp client connection
  -S, --tcp-server-output <<host>:<port>>
          serve output on <host>:<port>
  -P, --pretty-print
          pretty print json output (this format is not valid for backward conversion)
  -h, --help
          Print help
  -V, --version
          Print version
```

## Building the Executable

To build the executable, run:

```
cargo build --release
```

This will compile the application and create an executable in the `target/release/` directory.

Or you can build and install it with cargo:

```
cargo install rtcm-json
```

## License

MIT or Apache-2.0
