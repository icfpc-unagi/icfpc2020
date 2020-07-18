use app::*;
use draw::draw;
use modulation::demodulate;
use num::*;
use parser::E;
use std::io::*;

// Usage: draw output.png <<< 111101001000
fn main() {
	let args: Vec<_> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("1 arg required");
		std::process::exit(1);
	}
	let output = &args[1];
	let stdin = std::io::stdin();
	let stdin = stdin.lock();
	for line in stdin.lines() {
		let line = line.unwrap();
		let result = demodulate(&line);
		let img = draw(&translate(&result));
		img.save(output).unwrap();
	}
}

fn translate(e: &E) -> Vec<(BigInt, BigInt)> {
	let mut i = e;
	let mut out = Vec::new();
	while let E::Pair(head, tail) = &*i {
		if let E::Pair(x, y) = head.as_ref() {
			if let E::Num(x) = x.as_ref() {
				if let E::Num(y) = y.as_ref() {
					out.push((x.clone(), y.clone()));
					i = tail.as_ref();
				}
			}
		}
	}
	out
}
