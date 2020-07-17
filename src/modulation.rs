use num::bigint::Sign;
use num::*;
use std::fmt::*;
use std::iter::*;
use std::rc::Rc;
use std::string::*;

use super::parser::E;

pub fn modulate(e: &E) -> String {
  modulate_mod(&translate(e))
}

#[derive(Clone, PartialEq, Eq)]
enum Mod {
  Nil,
  Num(BigInt),
  Pair(Box<Mod>, Box<Mod>),
}

fn translate(e: &E) -> Mod {
  match e {
    E::Etc(x) if x == "nil" => Mod::Nil,
    E::Num(i) => Mod::Num(i.clone()),
    E::Pair(a, b) => Mod::Pair(
      Box::new(translate(a.as_ref())),
      Box::new(translate(b.as_ref())),
    ),
    _ => panic!(),
  }
}

fn modulate_mod(m: &Mod) -> String {
  match m {
    Mod::Nil => String::from("00"),
    Mod::Pair(a, b) => modulate_mod(a) + &modulate_mod(b),
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

pub fn demodulate_num(s: &str) -> BigInt {
  BigInt::from_biguint(
    match &s[0..2] {
      "01" => Sign::Plus,
      "10" => Sign::Minus,
      _ => panic!(),
    },
    BigUint::from_str_radix(&s[2 + s[2..].find(|x| x == '0').unwrap()..], 2).unwrap(),
  )
}

#[test]
fn test_modulate_mod() {
  assert_eq!(modulate_mod(&Mod::Num(Zero::zero())), "010");
  assert_eq!(modulate_mod(&Mod::Num(One::one())), "01100001");
  assert_eq!(
    modulate_mod(&Mod::Num(BigInt::new(Sign::Minus, vec![16]))),
    "1011000010000"
  );
}

#[test]
fn test_demodulate_num() {
  assert_eq!(demodulate_num("010"), Zero::zero());
  assert_eq!(demodulate_num("01100001"), One::one());
  assert_eq!(
    demodulate_num("1011000010000"),
    BigInt::new(Sign::Minus, vec![16])
  );
}
