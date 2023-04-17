use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("andre-i.eu:80").unwrap();
    stream
        .write("GET http://azenv.net/ HTTP/1.1\r\n".as_bytes())
        .unwrap();
    stream.write("Host: azenv.net\r\n".as_bytes()).unwrap();
    stream
        .write("User-Agent: Go-http-client/1.1\r\n".as_bytes())
        .unwrap();
    stream.write("\r\n".as_bytes()).unwrap();
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();
    println!("{}", buffer);
    println!(
        "\r\nyour server is: {}\n\n",
        if buffer.contains("<title>403 Forbidden</title>") {
            "SECURE"
        } else {
            "UNSECURE"
        }
    );
}
