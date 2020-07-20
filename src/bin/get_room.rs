extern crate colored;

use std::process::*;
use std::io::prelude::*;
use colored::*;
use app::client::*;
use app::parser::*;

fn get_room(server_url: String) -> (String, String) {
	let client = Client::new(server_url);
	let e = get_list(&client.send("[1, 0]")).unwrap()[1].clone();
	let e = get_list(&e).unwrap();
	let a = get_list(&e[0]).unwrap();
	let d = get_list(&e[1]).unwrap();
	(a[1].to_string(), d[1].to_string())
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let server_url = args[1].clone();
	let client = Client::new(server_url);
	let e = get_list(&client.send("[1, 0]")).unwrap()[1].clone();
	let e = get_list(&e).unwrap();
	let a = get_list(&e[0]).unwrap();
	let d = get_list(&e[1]).unwrap();
	println!("{}", a[1]);
	println!("{}", d[1]);
}
