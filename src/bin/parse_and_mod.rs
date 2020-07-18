use std::io::prelude::*;

use app::*;
use modulation::modulate;
use parser::E;

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
	let mut data = app::parser::Data::default();
	for id in functions.keys() {
		let f = parser::eval(&functions[id], &functions, false, &mut data);
		println!("{}: {}", id, f);
	}
	let f = parser::eval(&functions["main"], &functions, false, &mut data);
	let result = parser::eval(&f, &functions, true, &mut data);
	if let E::Pair(_, x) = &result {
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
