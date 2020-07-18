use std::io::prelude::*;

use app::parser::*;
use std::rc::Rc;
use rand::prelude::*;

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
	let state = "ap ap cons 5 ap ap cons ap ap cons 1 ap ap cons 0 ap ap cons nil ap ap cons nil ap ap cons nil ap ap cons nil ap ap cons nil ap ap cons 0 nil ap ap cons 0 ap ap cons nil nil";
	let mut state = eval(&parse(&state.split_whitespace().collect::<Vec<_>>(), 0).0, &functions, true, &mut Default::default());
	eprintln!("{}", state);
	let mut rng = rand::thread_rng();
	let mut last_flag = 0;
	for iter in 0.. {
		let x = rng.gen_range(-20, 20);
		let y = rng.gen_range(-20, 20);
		let s = if last_flag == 0 { "ap ap cons 1 ap ap cons 80000 nil".to_owned() } else { format!("ap ap cons {} {}", x, y) };
		let xy = parse(&s.split_whitespace().collect::<Vec<_>>(), 0).0;
		// eprintln!("{}", last_flag);
		eprintln!("{}; {}, {}", state, x, y);
		let exp = E::Ap(
			Rc::new(E::Ap(Rc::new(E::Etc(":1338".to_owned())), state.clone().into())),
			xy.into(),
		);
		let mut data = app::parser::Data::default();
		let f = eval(&exp, &functions, true, &mut data);
		// let f = eval(&exp, &functions, false, &mut data);
		// let f = eval(&f, &functions, true, &mut data);

		// eprintln!("{}", f);
		// for (id, c) in data.count {
		// 	eprintln!("{}: {}", id, c);
		// }
		let (flag, new_state, data) = if let E::Pair(flag, a) = f {
			if let E::Pair(a, b) = a.as_ref() {
				if let E::Pair(data, _) = b.as_ref() {
					(flag.as_ref() != &E::Num(0.into()), a.as_ref().clone(), data.as_ref().clone())
				} else {
					panic!();
				}
			} else {
				panic!();
			}
		} else {
			panic!();
		};
		if flag || state.to_string() == "[]" || (state.to_string().len(), state.to_string()) < (new_state.to_string().len(), new_state.to_string()) {
			state = new_state;
			eprintln!("flag = {}", flag);
			eprintln!("{} {}", x, y);
			eprintln!("state: {}", state);
			eprintln!("data: {}", data);
			println!("modulated: {}", app::modulation::modulate(&data));
			if !flag {
				app::visualize::multidraw_from_e(&data);
			}
			eprintln!("state: {}", state);
		}
		if flag {
			last_flag = 0;
		} else {
			last_flag = -1;
		}
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
