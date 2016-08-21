extern crate mio;

use std::net::TcpStream;

fn main() {
    let mut _stream = TcpStream::connect("0.0.0.0:6567").unwrap();
}
