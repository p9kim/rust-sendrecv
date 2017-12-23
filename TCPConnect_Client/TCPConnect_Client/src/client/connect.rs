use std::io::prelude::*;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr, TcpStream};

pub fn invoke_client(server_ip_str: String, server_port_str: String, 
	op: String, filename: String)
{
	println!("IP is :{}...Port is : {} Op is : {}...File is : {}", 
		server_ip_str, server_port_str, op, filename);

	let connect_to = format!("{}:{}", server_ip_str, server_port_str);

	println!("Connecting to: {}\n", connect_to);
	

	let mut contents = String::new();
	let mut file = File::open(&filename).unwrap();
	file.read_to_string(&mut contents).unwrap();

	let to_send = format!("{} {}\r\n{}", op, filename, contents);
	println!("Sending: {}\n", to_send);

	let mut stream = TcpStream::connect(connect_to).unwrap();

	stream.write(to_send.as_bytes()).unwrap();
	stream.flush().unwrap();

}