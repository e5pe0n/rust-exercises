use std::{
    io::{self, BufRead, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

fn handle_client(stream: TcpStream) -> Result<(), io::Error> {
    let mut buf_reader = io::BufReader::new(&stream);
    let mut buf_writer = io::BufWriter::new(&stream);
    let mut buf = String::new();
    buf_reader.read_to_string(&mut buf)?;
    println!("{}", &buf);
    buf_writer.write(buf.as_bytes())?;
    buf_writer.flush()?;

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                _ = handle_client(stream);
            });
        } else {
            eprintln!("Bad connection");
            continue;
        }
    }
    Ok(())
}
