use std::env;
use std::net::TcpStream;
use std::io::Write;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3310")
        .expect("Failed to connect to coproc");

    for arg in env::args().skip(1) {
        stream.write(arg.as_bytes()).expect("Failed to write to stream");
    }

    stream.write(&[0xA]).expect("Failed to disconnect");
}
