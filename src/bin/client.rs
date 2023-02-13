use std::{
    io::{self, Read, Write},
    net::TcpStream,
    str,
};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1234")?;

    let msg = b"hello";
    stream.write_all(msg)?;

    let mut rbuf = [0; 64];
    let _ = stream.read(&mut rbuf)?;
    let s = str::from_utf8(&rbuf).unwrap();
    println!("{s}");
    Ok(())
}
