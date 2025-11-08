use std::{
    collections::hash_map::{Entry, HashMap},
    sync::Arc,
};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream, ToSocketAddrs, tcp::OwnedWriteHalf},
    sync::{Notify, mpsc},
    task,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

#[tokio::main]
pub(crate) async fn main() -> Result<()> {
    accept_loop("127.0.0.1:8080").await
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let (broker_sender, broker_receiver) = mpsc::unbounded_channel();
    let broker = task::spawn(broker_loop(broker_receiver));
    let shutdown_notification = Arc::new(Notify::new());

    loop {
        tokio::select! {
            Ok((stream, _socket_addr)) = listener.accept() => {
                println!("accepting from: {}", stream.peer_addr()?);
                spawn_and_log_error(connection_loop(broker_sender.clone(), stream, shutdown_notification.clone()));
            },
            _ = tokio::signal::ctrl_c() => break
        }
    }

    println!("shutting down server...");
    shutdown_notification.notify_waiters();
    drop(broker_sender);
    broker.await?;

    Ok(())
}

async fn connection_loop(
    broker: Sender<Event>,
    stream: TcpStream,
    shutdown: Arc<Notify>,
) -> Result<()> {
    let (reader, writer) = stream.into_split();
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

    let name = match lines.next_line().await {
        Ok(Some(line)) => line,
        Ok(None) => return Err("peer disconnected immediately")?,
        Err(e) => return Err(Box::new(e)),
    };

    println!("user {} connected", name);

    broker
        .send(Event::NewPeer {
            name: name.clone(),
            stream: writer,
        })
        .unwrap();

    loop {
        tokio::select! {
        Ok(Some(line)) = lines.next_line()=> {
            let (dest, msg) = match line.find(':') {
                None => continue,
                Some(idx) => (&line[..idx], line[idx + 1..].trim()),
            };
            let dest = dest
                .split(',')
                .map(|name| name.trim().to_string())
                .collect::<Vec<_>>();
            let msg = msg.to_string();

            broker
                .send(Event::Message {
                    from: name.clone(),
                    to: dest,
                    msg,
                })
                .unwrap();
        },
        _ = shutdown.notified() => break
        }
    }

    Ok(())
}

async fn connection_write_loop(
    messages: &mut Receiver<String>,
    stream: &mut OwnedWriteHalf,
) -> Result<()> {
    loop {
        let msg = messages.recv().await;
        match msg {
            Some(msg) => stream.write_all(msg.as_bytes()).await?,
            None => break,
        }
    }
    Ok(())
}

#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: OwnedWriteHalf,
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}

async fn broker_loop(mut events: Receiver<Event>) {
    let mut peers: HashMap<String, Sender<String>> = HashMap::new();

    loop {
        let event = match events.recv().await {
            Some(event) => event,
            None => break,
        };

        match event {
            Event::Message { from, to, msg } => {
                for addr in to {
                    if let Some(peer) = peers.get_mut(&addr) {
                        let msg = format!("from {from}: {msg}\n");
                        peer.send(msg).unwrap();
                    }
                }
            }
            Event::NewPeer { name, mut stream } => match peers.entry(name.clone()) {
                Entry::Occupied(..) => (),
                Entry::Vacant(entry) => {
                    let (client_sender, mut client_receiver) = mpsc::unbounded_channel();
                    entry.insert(client_sender);
                    spawn_and_log_error(async move {
                        connection_write_loop(&mut client_receiver, &mut stream).await
                    });
                }
            },
        }
    }

    drop(peers)
}

fn spawn_and_log_error<F: Future<Output = Result<()>> + Send + 'static>(
    fut: F,
) -> task::JoinHandle<()> {
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e);
        }
    })
}
