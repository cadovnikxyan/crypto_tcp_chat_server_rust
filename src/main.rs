
extern crate crypto_tcp_chat_server;
use crypto_tcp_chat_server::chat_server;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpStream};


fn main(){

	chat_server::server_start2();
  
        // start_server();

        // let mut sock = BufReader::new(TcpStream::connect(HOST).unwrap());
        // let mut recv = String::new();

        // sock.get_mut().write_all(b"hello world\n").unwrap();
        // sock.read_line(&mut recv).unwrap();

        // assert_eq!(recv, "hello world\n");

        // recv.clear();

        // sock.get_mut().write_all(b"this is a line\n").unwrap();
        // sock.read_line(&mut recv).unwrap();

        // assert_eq!(recv, "this is a line\n");
    }