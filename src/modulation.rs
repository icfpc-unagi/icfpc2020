use num::bigint::Sign;
use num::*;
use std::fmt::*;
use std::iter::*;
use std::string::*;

pub fn modulate_num(i: BigInt) -> String {
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
fn test_modulate_num() {
  assert_eq!(modulate_num(Zero::zero()), "010");
  assert_eq!(modulate_num(One::one()), "01100001");
  assert_eq!(
    modulate_num(BigInt::new(Sign::Minus, vec![16])),
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
