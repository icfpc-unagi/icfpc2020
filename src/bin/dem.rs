use std::io::*;

use app::*;
use modulation::demodulate;

fn main() {
	let stdin = std::io::stdin();
	let stdin = stdin.lock();
	for line in stdin.lines() {
		let line = line.unwrap();
		let result = demodulate(&line);
		println!("{}", result);
	}
}
