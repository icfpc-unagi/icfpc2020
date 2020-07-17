use num::*;
use std::option::Option;
use std::vec::Vec;
use core::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum ModElem {
    Nil,
    Cons,
    Number(BigInt),
}

impl fmt::Debug for ModElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModElem::Nil => write!(f, "nil"),
            ModElem::Cons => write!(f, "cons"),
            ModElem::Number(i) => write!(f, "{}", i),
        }
    }
}

pub fn demodulate(s: &str) -> Option<Vec<ModElem>> {
    let mut s = s;
    let mut results = vec![];

    while !s.is_empty() {
        if s.starts_with("00") {
            results.push(ModElem::Nil);
            s = &s[2..];
        } else if s.starts_with("11") {
            results.push(ModElem::Cons);
            s = &s[2..];
        } else {
            let sign = if s.starts_with("01") {
                1
            } else if s.starts_with("10") {
                -1
            } else {
                return None;
            };
            s = &s[2..];

            let len = match s.find("0") {
                Some(i) => i,
                None => return None,
            };
            s = &s[len + 1..];

            let mut result = BigInt::from(0);
            for _ in 0..len * 4 {
                result *= 2;
                if s.starts_with("1") {
                    result += 1;
                } else if !s.starts_with("0") {
                    return None;
                }
                s = &s[1..];
            }
            results.push(ModElem::Number(BigInt::from(sign) * result));
        }
    }

    return Some(results);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demodule() {
        // https://icfpcontest2020.github.io/#/post/2050
        assert_eq!(
            format!(
                "{:?}",
                demodulate("110110000111011111100001001111110101000000")
            ),
            "Some([cons, 1, cons, 81744, nil])"
        );
        assert_eq!(
            format!(
                "{:?}",
                demodulate("110110000111011111100001001111110100110000")
            ),
            "Some([cons, 1, cons, 81740, nil])"
        );
    }
}
