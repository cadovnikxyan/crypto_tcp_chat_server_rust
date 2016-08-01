extern crate crypto;
extern crate ini;

use std::collections::VecDeque;
use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use self::ini::Ini;


struct chat_client<'a>{
	 client: &'a TcpStream,
	 name : String,	
}

struct server_config{
	host :String,
	
}

fn init() ->server_config{
	let conf = Ini::load_from_file("config.ini").unwrap();
	let section = conf.section(Some("server".to_owned())).unwrap();
	let server : server_config = server_config{host:section.get("host").unwrap().to_string()};
	server
}

fn handler(stream:TcpStream){
	let mut buffer=String::new();
	let mut s:TcpStream=stream;
	s.read_to_string(&mut buffer);
	println!("read: {}",buffer);
	s.write_all(b"success!");
}

pub fn server_start(){
	
	let server= init();
	let mut clients : VecDeque<chat_client> = VecDeque::new();
	let host: &str = &server.host[..];
	println!("server is start on host: {}",&server.host);
	let listener = TcpListener::bind(&host).unwrap();

	for stream in listener.incoming(){
		match stream{
			Ok(stream)=>{
				thread::spawn(move||
										{
											println!("new connection!");
											handler(stream)
										});
									},
			Err(err)=>{
						break;
			},
		}
	}
drop(listener);
}
