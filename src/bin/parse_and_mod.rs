use std::io::prelude::*;

use app::*;
use modulation::modulate;

fn main() {
	let stdin = std::io::stdin();
	let stdin = stdin.lock();
	let mut functions = std::collections::BTreeMap::new();
	for line in stdin.lines() {
		let line = line.unwrap();
		let ss = line.split_whitespace().collect::<Vec<_>>();
		let name = ss[0].to_owned();
		let (exp, n) = parser::parse(&ss[2..], 0);
		assert_eq!(n, ss.len() - 2);
		functions.insert(name, exp);
	}
	for id in functions.keys() {
		let f = parser::eval(&functions[id], &functions, false);
		println!("{}: {}", id, f);
	}
	let f = parser::eval(&functions["hoge"], &functions, false);
	println!("ret: {}", modulate(&parser::eval(&f, &functions, true)));
}
