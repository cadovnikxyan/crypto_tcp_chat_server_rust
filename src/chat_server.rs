extern crate crypto;
extern crate ini;

extern crate mio;
extern crate bytes;

use self::mio::{TryRead, TryWrite};
use self::mio::tcp::*;
use self::mio::util::Slab;
use self::bytes::{Buf, Take};
use std::mem;
use std::net::SocketAddr;
use std::io::Cursor;


const SERVER: mio::Token = mio::Token(0);

struct Pong {
    server: TcpListener,
    connections: Slab<Connection>,
}

impl Pong {
    // Initialize a new `Pong` server from the given TCP listener socket
    fn new(server: TcpListener) -> Pong {
        // Token `0` is reserved for the server socket. Tokens 1+ are used for
        // client connections. The slab is initialized to return Tokens
        // starting at 1.
        let slab = Slab::new_starting_at(mio::Token(1), 1024);

        Pong {
            server: server,
            connections: slab,
        }
    }
}
impl mio::Handler for Pong {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut mio::EventLoop<Pong>, token: mio::Token, events: mio::EventSet) {
        match token {
            SERVER => {
                // Only receive readable events
                assert!(events.is_readable());

                println!("the server socket is ready to accept a connection");
                match self.server.accept() {
                    Ok(Some(socket)) => {
                        println!("accepted a socket, exiting program");
                        let token = self.connections
                            .insert_with(|token| Connection::new(socket, token))
                            .unwrap();
                        event_loop.register_opt(
                        &self.connections[token].socket,
                        token,
                        mio::EventSet::readable(),
                        mio::PollOpt::edge() | mio::PollOpt::oneshot()).unwrap();
                        // event_loop.shutdown();
                    }
                    Ok(None) => {
                        println!("the server socket wasn't actually ready");
                    }
                    Err(e) => {
                        println!("listener.accept() errored: {}", e);
                        event_loop.shutdown();
                    }
                }
            }
            _ =>{self.connections[token].ready(event_loop, events);

                // If handling the event resulted in a closed socket, then
                // remove the socket from the Slab. This will result in all
                // resources being freed.
                if self.connections[token].is_closed() {
                    let _ = self.connections.remove(token);}
                },
        }
    }

}


pub fn server_start2() {
    let address = "10.0.0.75:3333".parse().unwrap();
    let server = TcpListener::bind(&address).unwrap();

    let mut event_loop = mio::EventLoop::new().unwrap();
    event_loop.register(&server, SERVER);

    println!("running pingpong server");
    let mut pong = Pong::new(server);

    // Run the `Pong` server
    println!("running pingpong server; port=3333");
    event_loop.run(&mut pong).unwrap();
}