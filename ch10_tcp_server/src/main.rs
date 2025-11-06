use std::{
    io::{self, BufRead, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(stream: TcpStream) -> Result<(), io::Error> {
    let buf_reader = io::BufReader::new(&stream);
    let mut buf_writer = io::BufWriter::new(&stream);
    for line in buf_reader.lines() {
        if let Ok(line) = line {
            let resp = "> ".to_string() + &line + "\n";
            buf_writer.write(resp.as_bytes())?;
            buf_writer.flush()?;
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(|| {
                _ = handle_client(stream);
            });
        } else {
            eprintln!("Bad connection");
            continue;
        }
    }
    Ok(())
}
