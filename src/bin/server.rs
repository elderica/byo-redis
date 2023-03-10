use std::io::{self, ErrorKind, Read, Write};

use std::net::{TcpListener, TcpStream};

const MAX_MESSAGE: u32 = 4096;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    loop {
        let e = one_request(&mut stream);
        if e.is_err() {
            break;
        }
    }
    Ok(())
}

fn one_request(stream: &mut TcpStream) -> io::Result<()> {
    let mut proto_len = [0; 4];
    let r = stream.read_exact(&mut proto_len);
    if r.is_err() {
        let e = r.as_ref().unwrap_err();
        match e.kind() {
            ErrorKind::UnexpectedEof => eprintln!("EOF"),
            _ => eprintln!("read(proto_len) error: {e:?}"),
        }
        return r;
    }

    let len = u32::from_le_bytes(proto_len);
    if len > MAX_MESSAGE {
        eprintln!("too long");
        let e = std::io::Error::new(ErrorKind::Other, "too long");
        return io::Result::Err(e);
    }

    // request body
    let mut rbuf = vec![0; len as usize];
    let r = stream.read_exact(&mut rbuf);
    if r.is_err() {
        let e = r.as_ref().unwrap_err();
        eprintln!("read(request_body) error: {e:?}");
        return r;
    }

    // do something
    // let s: String = rbuf
    //     .iter()
    //     .map(|b| std::ascii::escape_default(*b).collect())
    //     .flat_map(String::from_utf8)
    //     .collect();
    let s = String::from_utf8_lossy(&rbuf).into_owned();
    println!("client says: {s}");

    // reply using the same protocol
    let reply = b"world";
    let len = reply.len();
    let mut wbuf = Vec::new();
    let proto_len = u32::to_le_bytes(len as u32);
    wbuf.extend_from_slice(&proto_len[..4]);
    wbuf.extend_from_slice(reply);

    stream.write_all(&wbuf)?;

    Ok(())
}
