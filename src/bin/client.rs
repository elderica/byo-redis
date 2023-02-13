use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpStream,
};

const MAX_MESSAGE: u32 = 4096;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:1234")?;

    query(&mut stream, b"hello1")?;
    query(&mut stream, b"hello2")?;
    query(&mut stream, b"hello3")?;

    Ok(())
}

fn query(stream: &mut TcpStream, text: &[u8]) -> io::Result<()> {
    let len = text.len();
    if len > MAX_MESSAGE as usize {
        let e = std::io::Error::new(ErrorKind::Other, "too long");
        return io::Result::Err(e);
    }

    let mut wbuf = Vec::new();
    let blen = u32::to_le_bytes(len as u32);
    wbuf.extend_from_slice(&blen[..4]);
    wbuf.extend_from_slice(&text[..len]);
    let r = stream.write_all(&wbuf);
    if r.is_err() {
        eprintln!("write_all(wbuf) error");
        return r;
    }

    let mut proto_len = [0; 4];
    let r = stream.read_exact(&mut proto_len);
    if r.is_err() {
        eprintln!("read(proto_size) error: {:?}", r.as_ref().err());
        return r;
    }

    let len = u32::from_le_bytes(proto_len);
    if len > MAX_MESSAGE {
        eprintln!("too long");
        let e = std::io::Error::new(ErrorKind::Other, "too long");
        return io::Result::Err(e);
    }

    // reply body
    let mut rbuf = vec![0; len as usize];
    let r = stream.read_exact(&mut rbuf);
    if r.is_err() {
        eprintln!("read(reply_body) error");
        return r;
    }

    let s = std::str::from_utf8(&rbuf).unwrap();
    println!("server says: {s}");

    Ok(())
}
