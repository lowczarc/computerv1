use crate::utils::fmt_number_with_vars;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    BadDegree,
    Invalid,
    OverflowPower
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadDegree => write!(
                f,
                "computerv1 can't solve polynomes of degree higher than 2"
            ),
            Self::Invalid => write!(f, "Invalid input"),
            Self::OverflowPower => write!(f, "The value of an exponent is greater than the maximal u32 value")
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Polynome(HashMap<u32, f64>);

impl Polynome {
    pub fn new(tab: HashMap<u32, f64>) -> Self {
        Self(tab.into_iter().filter(|elem| elem.1 != 0.).collect())
    }

    pub fn get_multiplier_degree(&self, degree: u32) -> f64 {
        *self.0.get(&degree).unwrap_or(&0.)
    }

    pub fn get_degree(&self) -> u32 {
        self.0
            .iter()
            .max_by(|a, b| a.0.cmp(b.0))
            .map_or(0, |a| *a.0)
    }

    fn parse_a(a: &str) -> f64 {
        if a == "-" {
            -1.
        } else if let Ok(res) = a.parse() {
            res
        } else {
            1.
        }
    }

    pub fn parse(input: &str) -> Result<Self, Error> {
        let parsing_regex = Regex::new(r"(?:(?:^|(?P<operator>\+|\-))(?:(?: *(?P<a1>-?\d+(?:\.\d+)?) *\* *(?:(?P<a2>-?(?:\d+(?:\.\d+)?)?)?(X|x)(?:\^(?P<b1>\d+))?) *)|(?: *(?:(?P<a3>-?(?:\d+(?:\.\d+)?)?)?(X|x)(?:\^(?P<b2>\d+))?) *)|(?: *(?P<a4>-?\d+(?:\.\d+)?) *)))").unwrap();
        let captures = parsing_regex.captures_iter(input);
        let mut ret = Self::new(HashMap::new());

        let mut total_len_captures = 0;
        for cap in captures {
            total_len_captures += cap.get(0).unwrap().as_str().len();
            let a1: f64 = cap.name("a1").map_or(1., |a| Self::parse_a(a.as_str()));
            let a2: f64 = cap.name("a2").map_or(1., |a| Self::parse_a(a.as_str()));
            let a3: f64 = cap.name("a3").map_or(1., |a| Self::parse_a(a.as_str()));
            let a4: f64 = cap.name("a4").map_or(1., |a| Self::parse_a(a.as_str()));
            let operator: f64 =
                cap.name("operator")
                    .map_or(1., |x| if x.as_str() == "-" { -1. } else { 1. });

            let mut a = a1 * a2 * a3 * a4 * operator;
            let b: u32 = if let Some(b) = cap.name("b1").or(cap.name("b2")) {
                if let Ok(x) = b.as_str().parse() {
                    x
                } else {
                    return Err(Error::OverflowPower);
                }
            } else if cap.name("a4").is_some() {
                0
            } else {
                1
            };

            if let Some(previous_a) = ret.0.get(&b) {
                a += previous_a;
            }

            ret.0.insert(b, a);
        }

        if total_len_captures != input.len() || total_len_captures == 0 {
            return Err(Error::Invalid);
        }

        ret.0 = ret.0.into_iter().filter(|elem| elem.1 != 0.).collect();

        return Ok(ret);
    }
}

impl Sub for Polynome {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        for (key, value) in other.0.iter() {
            let self_value = *self.0.get(key).unwrap_or(&0.);

            self.0.insert(*key, self_value - value);
        }
        self.0 = self.0.into_iter().filter(|elem| elem.1 != 0.).collect();
        self
    }
}

impl fmt::Display for Polynome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_polynome = self.0.iter().collect::<Vec<(&u32, &f64)>>();
        sorted_polynome.sort_unstable_by(|a, b| a.0.cmp(b.0));

        let ret = sorted_polynome
            .into_iter()
            .map(|elem| {
                (
                    *elem.1,
                    match elem.0 {
                        0 => "".into(),
                        1 => "X".into(),
                        x => format!("X^{}", x),
                    },
                )
            })
            .collect();

        write!(f, "{}", fmt_number_with_vars(ret))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_polynomes() {
        let polytest1 = Polynome::new(HashMap::new());
        assert_eq!(format!("{}", polytest1), "0");

        let mut hashmap = HashMap::new();
        hashmap.insert(0, 2.);
        hashmap.insert(1, 6.);
        hashmap.insert(2, 98.);
        let polytest2 = Polynome::new(hashmap);
        assert_eq!(format!("{}", polytest2), "2 + 6X + 98X^2");

        let mut hashmap = HashMap::new();
        hashmap.insert(0, -54.);
        hashmap.insert(1, 0.);
        hashmap.insert(2, -7.);
        let polytest3 = Polynome::new(hashmap);
        assert_eq!(format!("{}", polytest3), "-54 - 7X^2");
    }

    #[test]
    fn parse_polynomes() {
        let polytest1 = Polynome::parse("0").unwrap();
        let mut hashmap = HashMap::new();
        hashmap.insert(0, 0.);
        assert_eq!(polytest1, Polynome::new(hashmap));

        let polytest2 = Polynome::parse("2 + 47 * X^2").unwrap();
        let mut hashmap = HashMap::new();
        hashmap.insert(0, 2.);
        hashmap.insert(2, 47.);
        assert_eq!(polytest2, Polynome::new(hashmap));

        let polytest3 = Polynome::parse("-6 * X^0 + 12 * X^1 - -15 * X^2").unwrap();
        let mut hashmap = HashMap::new();
        hashmap.insert(0, -6.);
        hashmap.insert(1, 12.);
        hashmap.insert(2, 15.);
        assert_eq!(polytest3, Polynome::new(hashmap));

        let polytest4 = Polynome::parse("76.12 + 2.5X^0 - 6.7 * X^1 + 98.3 * X^2").unwrap();
        let mut hashmap = HashMap::new();
        hashmap.insert(0, 78.62);
        hashmap.insert(1, -6.7);
        hashmap.insert(2, 98.3);
        assert_eq!(polytest4, Polynome::new(hashmap));
    }
}
