
extern crate crypto_tcp_chat_server;
use crypto_tcp_chat_server::chat_server;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpStream};


fn main(){

	chat_server::server_start2();
  
      
    }