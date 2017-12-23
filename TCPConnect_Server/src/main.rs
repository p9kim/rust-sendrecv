use std::env;
mod server;

fn main() 
{
    let args: Vec<String> = env::args().collect();

    let config = Config::parse_config(&args);

    //server::function();

    server::connect::invoke_server(config.port, config.directory);
}

struct Config 
{
	port: String,
	directory: String,
}

impl Config 
{
	fn parse_config(args: &[String]) -> Config 
	{
		if args.len() > 3
		{
			panic!("too many arguments!");
		}

		let port = args[1].clone();
		let directory = args[2].clone();

		Config { port, directory }
	}
}

