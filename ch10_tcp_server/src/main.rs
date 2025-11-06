use std::{
    io::{self, BufRead, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

fn handle_client(stream: TcpStream, log_mutex: Arc<Mutex<Vec<usize>>>) -> Result<(), io::Error> {
    let buf_reader = io::BufReader::new(&stream);
    let mut buf_writer = io::BufWriter::new(&stream);
    for line in buf_reader.lines() {
        if let Ok(line) = line {
            let mut log = log_mutex.lock().unwrap();
            log.push(line.len());
            let resp = "> ".to_string() + &line + "\n";
            buf_writer.write(resp.as_bytes())?;
            buf_writer.flush()?;
            println!("{:?}", log);
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let log_mutex: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![]));

    for stream in listener.incoming() {
        {
            let log_mutex = Arc::clone(&log_mutex);
            if let Ok(stream) = stream {
                thread::spawn(move || {
                    _ = handle_client(stream, log_mutex);
                });
            } else {
                eprintln!("Bad connection");
                continue;
            }
        }
    }
    Ok(())
}
