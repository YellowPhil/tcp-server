use std::{error::Error, io};

use clap::Parser;
use mio::{Events, Interest, Poll, Token, net::TcpListener};
use tcp_protocol::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    let mut poll = Poll::new()?;

    let token = Token(0);

    let addr = config.connection_string.parse()?;
    let mut server = TcpListener::bind(addr)?;

    poll.registry()
        .register(&mut server, token, Interest::READABLE | Interest::WRITABLE)?;

    let mut events = Events::with_capacity(1024);
    loop {
        let _ = poll
            .poll(&mut events, None)
            .map_err(|e| eprintln!("Error waiting for events: {}", e));

        for event in events.iter() {
            if event.token() != token {
                continue;
            }
            match server.accept() {
                Ok((stream, _)) => {
                    let _ = tcp_protocol::server::handle_connection(stream)
                        .map_err(|e| eprintln!("Error handling connection: {}", e));
                }
                Err(e) => {
                    if e.kind() != io::ErrorKind::WouldBlock {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
