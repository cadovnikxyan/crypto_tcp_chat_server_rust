extern crate crypto;
extern crate ini;

use std::collections::HashMap;
use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use self::ini::Ini;
use self::crypto;

struct chat_client{
	 client:  TcpStream,
	 name : String,	
}

struct server_config{
	server_addr : String,
	server_port : String,
}

trait server_configuring{
	fn init(&mut self);
}

impl server_configuring for server_config{
	fn init(&mut self) {
		let conf = Ini::load_from_file("config.ini").unwrap();
		let section = conf.section(Some("server_init".to_owned())).unwrap();
		self.server_addr = section.get("server_addr").unwrap().to_string();
		self.server_port = section.get("server_port").unwrap().to_string();
	}
}

fn handler(stream:TcpStream){
	let mut buffer=String::new();
	let mut s:TcpStream=stream;
	s.read_to_string(&mut buffer);
	println!("read: {}",buffer);
	s.write_all(b"success!");
}

pub fn server_start(){
	let mut server = server_config {server_addr:"localhost:".to_string(),server_port:"33333".to_string()};
	// let mut server =server_config::init();
	server.init();
	// let mut clients : HashMap<chat_client,String> = HashMap::new();
	let listener = TcpListener::bind("localhost:3333").unwrap();
	for stream in listener.incoming(){
		match stream{
			Ok(stream)=>{
				thread::spawn(move||
									{
										println!("new connection!");
										handler(stream)});
									},
			Err(err)=>{
						break;
			},
		}
	}
drop(listener);
}
