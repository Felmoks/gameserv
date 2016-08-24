use std::io::Write;

fn main() {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:8888").unwrap();

    let str = b"-";

    let _ = stream.write(str);
}
