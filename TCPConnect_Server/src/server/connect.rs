use std::io::prelude::*;
use std::fs::File;
use std::net::{TcpListener, TcpStream};

pub fn invoke_server(server_port: String, directory: String) 
{
	println!("Creating Socket... 127.0.0.1:{}. Storing in {}", server_port, directory);

	let local = String::from("127.0.0.1");
	let server_ip = format!("{}:{}", local, server_port);

	println!("{}", server_ip);

	let listener = TcpListener::bind(server_ip).unwrap(); // try pattern matching


	for stream in listener.incoming()
	{
		let stream = stream.unwrap();
		//println!("In for loop");
		handle_connection(stream, &directory);
	}
}

fn handle_connection(mut stream: TcpStream, directory: & String)
{
	//println!("In handle_connection");
	let mut stringBuf = String::new();
	let mut amt_recvd = stream.read_to_string(&mut stringBuf).unwrap();

	while amt_recvd > 0
	{
		println!("In the loop: {} \n", stringBuf);

		amt_recvd = stream.read_to_string(&mut stringBuf).unwrap();
	}

	println!("Whole:{} \n", stringBuf);

	let header_split = stringBuf.find("\r\n").unwrap() + 2;
	let file_content = stringBuf.split_off(header_split);
	let header: Vec<&str> = stringBuf.split_whitespace().collect();
	let filepath = format!("{}", header[1]);
	let op = header[0];

	println!("File is:{}", header[1]);
	println!("Content is:{}", file_content);
	
	if op.eq("STOR")
	{
		println!("STOR");
		let mut file = File::create(&filepath).unwrap();

		file.write_all(file_content.as_bytes());
		file.sync_all();
	}

	let mut readTest = File::open(&filepath).unwrap();
	let mut buf = String::new();
	readTest.read_to_string(&mut buf).unwrap();
	println!("File Test: {}", buf);

	let response = String::from("HEEHEE");


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


//let mut file = File::open("test.html").unwrap();

	//let mut contents = String::new();
	//file.read_to_string(&mut contents).unwrap();

	//let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
	//println!("Testing:{}", stringBuf);