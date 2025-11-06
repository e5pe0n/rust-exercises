use std::{
    collections::VecDeque,
    io::{self, BufRead, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const DEFAULT_TIMEOUT: Option<Duration> = Some(Duration::from_millis(1000));

fn handle_client(stream: TcpStream, q: Arc<Mutex<VecDeque<String>>>) -> Result<(), io::Error> {
    stream.set_read_timeout(DEFAULT_TIMEOUT)?;
    stream.set_write_timeout(DEFAULT_TIMEOUT)?;

    let mut buf_reader = io::BufReader::new(&stream);
    let mut buf_writer = io::BufWriter::new(&stream);
    let mut buf = String::new();
    buf_reader.read_to_string(&mut buf)?;
    println!("received: {}", &buf);

    let res = simple_db::parse(&buf);
    match res {
        Ok(cmd) => match cmd {
            simple_db::Command::PUBLISH(payload) => {
                writeln!(buf_writer, "{}", &payload)?;
                println!("PUBLISH({})", &payload);
                q.lock().unwrap().push_back(payload);
            }
            simple_db::Command::RETRIEVE => {
                if let Some(s) = q.lock().unwrap().pop_front() {
                    writeln!(buf_writer, "{}", &s)?;
                    println!("RETRIEVE({})", &s);
                } else {
                    writeln!(buf_writer, "Error: empty queue")?;
                }
            }
        },
        Err(err) => {
            writeln!(buf_writer, "Error: {:?}", &err)?;
            println!("Error: {:?}", &err);
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let q: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let q = Arc::clone(&q);
            thread::spawn(move || {
                _ = handle_client(stream, q);
            });
        } else {
            eprintln!("Bad connection");
            continue;
        }
    }
    Ok(())
}
