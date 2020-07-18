use num::bigint::Sign;
use num::*;
use std::boxed::*;
use std::convert::{From, Into};
use std::fmt::*;
use std::iter::*;
use std::rc::Rc;
use std::string::*;

use super::parser::E;

pub fn modulate(e: &E) -> String {
	modulate_mod(&Mod::from(e))
}

pub fn demodulate(s: &str) -> E {
	(&demodulate_mod(s).0).into()
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Mod {
	Nil,
	Num(BigInt),
	Pair(Box<Mod>, Box<Mod>),
}

impl From<&E> for Mod {
	fn from(e: &E) -> Self {
		match e {
			E::Etc(x) if x == "nil" => Mod::Nil,
			E::Num(i) => Mod::Num(i.clone()),
			E::Pair(a, b) => Mod::Pair(
				Box::new(Mod::from(a.as_ref())),
				Box::new(Mod::from(b.as_ref())),
			),
			_ => panic!(),
		}
	}
}

impl Into<E> for &Mod {
	fn into(self) -> E {
		match self {
			Mod::Nil => E::Etc(String::from("nil")),
			Mod::Num(i) => E::Num(i.clone()),
			Mod::Pair(a, b) => E::Pair(Rc::new(a.as_ref().into()), Rc::new(b.as_ref().into())),
		}
	}
}

fn modulate_mod(m: &Mod) -> String {
	match m {
		Mod::Nil => String::from("00"),
		Mod::Pair(a, b) => String::from("11") + &modulate_mod(a) + &modulate_mod(b),
		Mod::Num(i) => {
			let mut s = String::new();
			s += match i.sign() {
				Sign::Minus => "10",
				Sign::NoSign => return String::from("010"),
				Sign::Plus => "01",
			};
			s += &"1".repeat(((i.bits() + 3) / 4) as usize);
			s += "0";
			i.magnitude().to_radix_be(16).iter().for_each(|x| {
				write!(&mut s, "{:04b}", x).unwrap();
			});
			s
		}
	}
}

fn demodulate_mod(s: &str) -> (Mod, usize) {
	match &s[0..2] {
		"00" => (Mod::Nil, 2),
		"11" => {
			let (a, p) = demodulate_mod(&s[2..]);
			let (b, q) = demodulate_mod(&s[2 + p..]);
			(Mod::Pair(Box::new(a), Box::new(b)), 2 + p + q)
		}
		"01" => {
			let (mag, p) = demodulate_uint(&s[2..]);
			(Mod::Num(BigInt::from_biguint(Sign::Plus, mag)), 2 + p)
		}
		"10" => {
			let (mag, p) = demodulate_uint(&s[2..]);
			(Mod::Num(BigInt::from_biguint(Sign::Minus, mag)), 2 + p)
		}
		_ => panic!(),
	}
}

fn demodulate_uint(s: &str) -> (BigUint, usize) {
	let p = s.find(|x| x == '0').unwrap();
	let end = p + 1 + p * 4;
	(
		if p == 0 {
			Zero::zero()
		} else {
			BigUint::from_str_radix(&s[p + 1..end], 2).unwrap()
		},
		end,
	)
}

#[test]
fn test_modulate_mod() {
	assert_eq!(modulate_mod(&Mod::Nil), "00");
	assert_eq!(
		modulate_mod(&Mod::Pair(Box::new(Mod::Nil), Box::new(Mod::Nil))),
		"110000"
	);
	assert_eq!(modulate_mod(&Mod::Num(Zero::zero())), "010");
	assert_eq!(modulate_mod(&Mod::Num(One::one())), "01100001");
	eprintln!("{}", modulate_mod(&Mod::Pair(Box::new(Mod::Num(1.into())), Box::new(Mod::Nil))));
	assert_eq!(
		modulate_mod(&Mod::Num(BigInt::new(Sign::Minus, vec![16]))),
		"1011000010000"
	);
	for i in -1000..1000 {
		assert_eq!(demodulate_mod(&modulate_mod(&Mod::Num(i.into()))).0, Mod::Num(BigInt::from(i)));
	}
}

#[test]
fn test_demodulate_num() {
	assert_eq!(demodulate_mod("00").0, Mod::Nil);
	assert_eq!(
		demodulate_mod("110000").0,
		Mod::Pair(Box::new(Mod::Nil), Box::new(Mod::Nil))
	);
	assert_eq!(demodulate_mod("010").0, Mod::Num(Zero::zero()));
	assert_eq!(demodulate_mod("01100001").0, Mod::Num(One::one()));
	assert_eq!(
		demodulate_mod("1011000010000").0,
		Mod::Num(BigInt::new(Sign::Minus, vec![16]))
	);
}
