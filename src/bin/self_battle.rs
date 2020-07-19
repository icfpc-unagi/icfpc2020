extern crate colored;

use std::process::*;
use std::io::prelude::*;
use colored::*;
use app::client::*;
use app::parser::*;

fn get_room(server_url: String) -> (String, String) {
	let client = Client::new(server_url);
	client.send("[1, 0]");
	let e = get_list(&client.send("[1, 0]")).unwrap()[1].clone();
	let e = get_list(&e).unwrap();
	let a = get_list(&e[0]).unwrap();
	let d = get_list(&e[1]).unwrap();
	(a[1].to_string(), d[1].to_string())
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let server_url = args[1].clone();
	let player1 = args[2].clone();
	let player2 = if args.len() < 4 { player1.clone() } else { args[3].clone() };
	let (a, d) = get_room(server_url.clone());
	let mut player1 = std::process::Command::new(format!("target/release/{}", player1)).args(&[server_url.clone(), a]).stdin(Stdio::piped()).stderr(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
	let mut player2 = std::process::Command::new(format!("target/release/{}", player2)).args(&[server_url.clone(), d]).stdin(Stdio::piped()).stderr(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
	{
		let player1_out = std::io::BufReader::new(player1.stdout.as_mut().unwrap());
		let player2_out = std::io::BufReader::new(player2.stdout.as_mut().unwrap());
		let player1_err = std::io::BufReader::new(player1.stderr.as_mut().unwrap());
		let player2_err = std::io::BufReader::new(player2.stderr.as_mut().unwrap());
		crossbeam::scope(|scope| {
			let h1 = scope.spawn(|_| {
				for line in player1_out.lines() {
					let line = line.unwrap();
					println!("{}", format!("{}", line).cyan());
				}
			});
			let h2 = scope.spawn(|_| {
				for line in player1_err.lines() {
					let line = line.unwrap();
					println!("{}", format!("{}", line).cyan());
				}
			});
			let h3 = scope.spawn(|_| {
				for line in player2_out.lines() {
					let line = line.unwrap();
					println!("{}", format!("{}", line).green());
				}
			});
			let h4 = scope.spawn(|_| {
				for line in player2_err.lines() {
					let line = line.unwrap();
					println!("{}", format!("{}", line).green());
				}
			});
			h1.join().unwrap();
			h2.join().unwrap();
			h3.join().unwrap();
			h4.join().unwrap();
		}).unwrap();
	}
	player1.wait().unwrap();
	player2.wait().unwrap();
}
