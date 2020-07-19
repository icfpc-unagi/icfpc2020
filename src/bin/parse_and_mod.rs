use std::io::prelude::*;

use app::*;
use modulation::modulate;
use parser::E;

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
	let mut data = app::parser::Data::default();
	let yabai = [1096, 1433, 1434, 1435, 1436, 1437];
	let yabai: ::std::collections::BTreeSet<String> = yabai.iter().map(
		|&n| format!(":{}", n)
	).collect();
	for id in functions.keys() {
		eprintln!("{}", id);
		let f = parser::eval(&functions[id], &functions, false, &mut data);
		let f = if yabai.contains(id) {
			f
		} else {
			parser::eval(&f, &functions, true, &mut data)
		};
		println!("{}: {}", id, f);
	}
	let f = parser::eval(&functions["main"], &functions, true, &mut data);
	if let E::Pair(_, x) = &f {
		if let E::Pair(state, x) = x.as_ref() {
			if let E::Pair(x, _) = x.as_ref() {
				println!("state: {}", state);
				println!("ret: {}", x);
				println!("state modulated: {}", modulate(&*state));
				println!("ret modulated: {}", modulate(&*x));
			} else {
				panic!();
			}
		} else {
			panic!();
		}
	} else {
		panic!();
	}
}

fn main() {
	let _ = ::std::thread::Builder::new()
		.name("run".to_string())
		.stack_size(32 * 1024 * 1024)
		.spawn(run)
		.unwrap()
		.join();
}
