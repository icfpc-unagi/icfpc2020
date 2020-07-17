use std::io::prelude::*;

use app::*;

fn run() {
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
	println!("ret: {}", parser::eval(&f, &functions, true));
	// let g = eval(&functions[":1108"], &functions);
	// let g = eval(&functions["galaxy"], &functions);
	// println!("{}", g);
	// let mut f = functions[":1141"].clone();
	// for _ in 0..100 {
	// 	f = simplify(&f);
	// }
	// println!("{}", f);
}

fn main() {
	let _ = ::std::thread::Builder::new()
		.name("run".to_string())
		.stack_size(32 * 1024 * 1024)
		.spawn(run)
		.unwrap()
		.join();
}
