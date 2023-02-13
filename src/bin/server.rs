use std::{
    io::{self, Read, Write},
    str,
};

use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;

    for stream in listener.incoming() {
        do_something(stream?);
    }

    Ok(())
}

fn do_something(mut stream: TcpStream) {
    let mut rbuf = [0; 64];
    let size = stream.read(&mut rbuf);
    if size.is_err() {
        eprintln!("read() error");
        return;
    }
    let s = str::from_utf8(&rbuf).unwrap();
    println!("client says: {s}");

    let wbuf = b"world";
    stream.write_all(wbuf).unwrap();
}
