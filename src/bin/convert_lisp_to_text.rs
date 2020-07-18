use std::io::*;

use app::*;

fn main() {
	let stdin = std::io::stdin();
	let stdin = stdin.lock();
	for line in stdin.lines() {
		let line = line.unwrap();
		let r = parser::parse_lisp(&line);
		println!("{}", parser::to_text(&r.0));
	}
}
