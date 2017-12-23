use std::env;
mod client;

fn main() 
{
	let args: Vec<String> = env::args().collect();

	let config = Config::parse_config(&args);

    client::connect::invoke_client(config.ip, config.port, config.op, config.filename);
}

struct Config
{
	op: String,
	ip: String,
	port: String,
	filename: String,
}

impl Config
{
	fn parse_config(args: &[String]) -> Config 
	{
		if args.len() != 5
		{
			panic!("Usage: ip port op filename");
		}

		let ip = args[1].clone();
		let port = args[2].clone();
		let op = args[3].clone();
		let filename = args[4].clone();
		/*
		match filename 
		{
			None => None,
			Some(i) => Some(i),
		}
		*/

		Config { op: op, ip: ip, port: port, filename: filename }
	}
}