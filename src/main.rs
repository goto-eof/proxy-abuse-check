use clap::{arg, command, Parser};
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

fn main() {
    println!("write -help for help");
    let args = Args::parse();
    let mut stream = TcpStream::connect_timeout(
        &format!("{}:{}", args.website, args.port)
            .to_socket_addrs()
            .unwrap()
            .into_iter()
            .next()
            .unwrap(),
        Duration::from_millis(10000),
    )
    .unwrap();
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
    println!(
        "\r\nyour server is: {}\n\n",
        if buffer.contains("<title>403 Forbidden</title>") {
            "SECURE"
        } else {
            "UNSECURE"
        }
    );
}

/// Proxy abuse check applciation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// website
    #[arg(short, long)]
    website: String,
    /// ports number
    #[arg(short, long)]
    port: u16,
}
