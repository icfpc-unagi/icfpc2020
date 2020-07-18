use num::*;
use std::collections::*;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum E {
	Ap(Rc<E>, Rc<E>),
	Num(BigInt),
	Pair(Rc<E>, Rc<E>),
	Etc(String),
}

pub fn parse(ss: &[&str], i: usize) -> (E, usize) {
	if ss[i] == "ap" {
		let (first, j) = parse(ss, i + 1);
		let (second, k) = parse(ss, j);
		(E::Ap(Rc::new(first), Rc::new(second)), k)
	// } else if ss[i] == "(" {
	// 	let mut list = vec![];
	// 	let mut i = i + 1;
	// 	while ss[i] != ")" {
	// 		let (e, j) = parse(ss, i);
	// 		list.push(e);
	// 		i = j;
	// 		if ss[i] == "," {
	// 			i += 1;
	// 		}
	// 	}
	// 	(E::List(list), i + 1)
	} else if let Ok(a) = ss[i].parse::<BigInt>() {
		(E::Num(a), i + 1)
	} else {
		(E::Etc(ss[i].to_owned()), i + 1)
	}
}

pub fn eval(e: &E, map: &BTreeMap<String, E>, eval_tuple: bool) -> E {
	match e {
		E::Ap(x1, y1) => {
			let x1 = eval(&x1, map, eval_tuple);
			match &x1 {
				E::Ap(x2, y2) => match x2.as_ref() {
					E::Etc(name) if name == "cons" => {
						if eval_tuple {
							E::Pair(
								eval(y2, map, eval_tuple).into(),
								eval(y1, map, eval_tuple).into(),
							)
						} else {
							E::Pair(y2.clone(), y1.clone().into())
						}
					}
					E::Etc(name) if name == "eq" => {
						let y1 = eval(&y1, map, eval_tuple);
						let y2 = eval(&y2, map, eval_tuple);
						match (&y1, &y2) {
							(E::Num(y1), E::Num(y2)) => {
								if y1 == y2 {
									E::Etc("t".to_owned())
								} else {
									E::Etc("f".to_owned())
								}
							}
							_ => {
								eprintln!("y1 = {}", y1);
								eprintln!("y2 = {}", y2);
								panic!();
							}
						}
					}
					E::Etc(name) if name == "t" => eval(&y2, map, eval_tuple),
					E::Etc(name) if name == "f" => eval(&y1, map, eval_tuple),
					E::Etc(name) if name == "add" => {
						let y1 = eval(&y1, map, eval_tuple);
						let y2 = eval(&y2, map, eval_tuple);
						match (y1, y2) {
							(E::Num(y1), E::Num(y2)) => E::Num(y1 + y2),
							_ => {
								panic!();
							}
						}
					}
					E::Etc(name) if name == "mul" => {
						let y1 = eval(&y1, map, eval_tuple);
						let y2 = eval(&y2, map, eval_tuple);
						match (&y1, &y2) {
							(E::Num(y1), E::Num(y2)) => E::Num(y1 * y2),
							_ => {
								eprintln!("y1 = {}", y1);
								eprintln!("y2 = {}", y2);
								panic!();
							}
						}
					}
					E::Etc(name) if name == "div" => {
						let y1 = eval(&y1, map, eval_tuple);
						let y2 = eval(&y2, map, eval_tuple);
						match (&y1, &y2) {
							(E::Num(y1), E::Num(y2)) => E::Num(y2 / y1),
							_ => {
								eprintln!("y1 = {}", y1);
								eprintln!("y2 = {}", y2);
								panic!();
							}
						}
					}
					E::Etc(name) if name == "lt" => {
						let y1 = eval(&y1, map, eval_tuple);
						let y2 = eval(&y2, map, eval_tuple);
						match (&y1, &y2) {
							(E::Num(y1), E::Num(y2)) => {
								if y2 < y1 {
									E::Etc("t".to_owned())
								} else {
									E::Etc("f".to_owned())
								}
							}
							_ => {
								eprintln!("y1 = {}", y1);
								eprintln!("y2 = {}", y2);
								panic!();
							}
						}
					}
					E::Ap(x3, y3) => match x3.as_ref() {
						E::Etc(name) if name == "b" => eval(
							&E::Ap(y3.clone(), Rc::new(E::Ap(y2.clone(), y1.clone()))),
							map,
							eval_tuple,
						),
						E::Etc(name) if name == "c" => eval(
							&E::Ap(Rc::new(E::Ap(y3.clone(), y1.clone())), y2.clone()),
							map,
							eval_tuple,
						),
						E::Etc(name) if name == "s" => eval(
							&E::Ap(
								Rc::new(E::Ap(y3.clone(), y1.clone())),
								Rc::new(E::Ap(y2.clone(), y1.clone())),
							),
							map,
							eval_tuple,
						),
						_ => E::Ap(Rc::new(x1), y1.clone()),
					},
					_ => E::Ap(x1.clone().into(), y1.clone().into()),
				},
				E::Pair(a, b) => eval(
					&E::Ap(Rc::new(E::Ap(y1.clone(), a.clone())), b.clone()),
					map,
					eval_tuple,
				),
				E::Etc(name) if name == "inc" => {
					if let E::Num(a) = eval(y1, map, eval_tuple) {
						E::Num(a + 1)
					} else {
						panic!();
					}
				}
				E::Etc(name) if name == "dec" => {
					if let E::Num(a) = eval(y1, map, eval_tuple) {
						E::Num(a - 1)
					} else {
						panic!();
					}
				}
				E::Etc(name) if name == "neg" => {
					if let E::Num(a) = eval(y1, map, eval_tuple) {
						E::Num(-a)
					} else {
						panic!();
					}
				}
				E::Etc(name) if name == "car" => {
					if let E::Pair(a, _) = eval(y1, map, eval_tuple) {
						eval(&a, map, eval_tuple)
					} else {
						panic!();
					}
				}
				E::Etc(name) if name == "cdr" => {
					if let E::Pair(_, a) = eval(y1, map, eval_tuple) {
						eval(&a, map, eval_tuple)
					} else {
						panic!();
					}
				}
				E::Etc(name) if name == "isnil" => {
					let y1 = eval(y1, map, eval_tuple);
					if let E::Etc(name) = y1 {
						if name == "nil" {
							E::Etc("t".to_owned())
						} else {
							E::Etc("f".to_owned())
						}
					} else if let E::Pair(_, _) = y1 {
						E::Etc("f".to_owned())
					} else {
						eprintln!("y1 = {}", y1);
						panic!();
					}
				}
				E::Etc(name) if name == "i" => eval(y1.as_ref(), map, eval_tuple),
				_ => E::Ap(Rc::new(x1), y1.clone().into()),
			}
		}
		E::Etc(name) if name.starts_with(":") => eval(&map[name], map, eval_tuple),
		E::Pair(a, b) if eval_tuple => E::Pair(
			eval(a, map, eval_tuple).into(),
			eval(b, map, eval_tuple).into(),
		),
		e => e.clone(),
	}
}

pub fn simplify(e: &E) -> E {
	match e {
		E::Ap(x1, y1) => {
			let x1 = simplify(x1);
			let y1 = simplify(y1);
			match &x1 {
				E::Etc(name) if name == "i" => y1,
				E::Ap(x2, y2) => match x2.as_ref() {
					E::Ap(x3, y3) => match x3.as_ref() {
						E::Etc(name) if name == "b" => {
							E::Ap(y3.clone(), Rc::new(E::Ap(y2.clone(), Rc::new(y1))))
						}
						E::Etc(name) if name == "c" => {
							E::Ap(Rc::new(E::Ap(y3.clone(), Rc::new(y1))), y2.clone())
						}
						E::Etc(name) if name == "s" => E::Ap(
							Rc::new(E::Ap(y3.clone(), Rc::new(y1.clone()))),
							Rc::new(E::Ap(y2.clone(), Rc::new(y1))),
						),
						_ => E::Ap(Rc::new(x1), Rc::new(y1)),
					},
					_ => E::Ap(Rc::new(x1), Rc::new(y1)),
				},
				_ => E::Ap(Rc::new(x1), Rc::new(y1)),
			}
		}
		_ => e.clone(),
	}
}

pub fn get_list(e: &E) -> Option<Vec<Rc<E>>> {
	let mut list = vec![];
	let mut e = e;
	while let E::Pair(a, b) = e {
		list.push(a.clone());
		e = b;
	}
	if e == &E::Etc("nil".to_owned()) {
		Some(list)
	} else {
		None
	}
}

impl std::fmt::Display for E {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			E::Ap(a, b) => {
				write!(f, "(")?;
				a.fmt(f)?;
				write!(f, " ")?;
				b.fmt(f)?;
				write!(f, ")")?;
			}
			E::Num(a) => write!(f, "{}", a)?,
			E::Pair(a, b) => {
				if let Some(list) = get_list(&self) {
					write!(f, "[")?;
					for i in 0..list.len() {
						if i > 0 {
							write!(f, ", ")?;
						}
						write!(f, "{}", list[i])?;
					}
					write!(f, "]")?;
				} else {
					write!(f, "<{}, {}>", a, b)?
				}
			}
			E::Etc(name) if name == "nil" => write!(f, "[]")?,
			E::Etc(name) => write!(f, "{}", name)?,
		}
		Ok(())
	}
}
