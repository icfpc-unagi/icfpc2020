use std::io::prelude::*;

use app::parser::*;
use std::rc::Rc;

fn run() {
	let stdin = std::io::stdin();
	let stdin = stdin.lock();
	let mut functions = std::collections::BTreeMap::new();
	for line in stdin.lines() {
		let line = line.unwrap();
		let ss = line.split_whitespace().collect::<Vec<_>>();
		let name = ss[0].to_owned();
		let (exp, n) = parse(&ss[2..], 0);
		assert_eq!(n, ss.len() - 2);
		functions.insert(name, exp);
	}
	// for id in functions.keys() {
	// 	let f = eval(&functions[id], &functions, false);
	// 	println!("{}: {}", id, f);
	// }
	let mut state = E::Etc("nil".to_owned());
	for (x, y) in vec![(-1, -3), (0, -3), (1, -3), (2, -2), (0, 0)] {
		let s = format!("ap ap cons {} {}", x, y);
		let xy = parse(&s.split_whitespace().collect::<Vec<_>>(), 0).0;
		let exp = E::Ap(Rc::new(E::Ap(Rc::new(E::Etc(":1338".to_owned())), state.into())), xy.into());
		let f = eval(&exp, &functions, false);
		let f = eval(&f, &functions, true);
		eprintln!("{}", f);
		state = if let E::Pair(_, a) = f {
			if let E::Pair(a, _) = a.as_ref() {
				a.as_ref().clone()
			} else {
				panic!();
			}
		} else {
			panic!();
		};
		eprintln!("state: {}", state);
	}
	// let f = eval(&functions["hoge"], &functions, false);
	// let f = eval(&f, &functions, true);
	// println!("ret: {}", f);
	
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
