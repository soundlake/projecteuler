use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Fraction {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    pub numerator: u32,
    pub denominator: u32,
}
impl Fraction {
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Option<Self> {
        let fraction = Fraction {
            a, b, c, d,
            numerator: a * 10 + b,
            denominator: c * 10 + d,
        };
        if fraction.is_verified() {
            Some(fraction)
        } else {
            None
        }
    }

    fn numerator(&self) -> u32 {
        self.a * 10 + self.b
    }

    fn denominator(&self) -> u32 {
        self.c * 10 + self.d
    }

    fn is_verified(&self) -> bool {
        // always two digits
        (self.a > 0 && self.c > 0)
            &&
            // each of them should be single digit
            (self.a < 10 || self.b < 10 || self.c < 10 || self.d < 10)
            &&
            // the numerator should be less than the denominator
            (self.numerator < self.denominator)
    }

    pub fn check(&self) -> bool {
        let numerator = self.a * 10 + self.b;
        let denominator = self.c * 10 + self.d;
        (numerator * self.d == denominator * self.a
            || numerator * self.c == denominator * self.b)
            && (self.a == self.d || self.b == self.c)
    }
}
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}/{}{}", self.a, self.b, self.c, self.d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        assert_eq!(None, Fraction::new(10, 1, 9, 9));
        assert_eq!(None, Fraction::new(0, 0, 9, 9));
        assert_eq!(None, Fraction::new(1, 1, 0, 0));
        assert_eq!("49/98", Fraction::new(4, 9, 9, 8).unwrap().to_string());
    }

    #[test]
    fn test_check() {
        assert_eq!(true, Fraction::new(4, 9, 9, 8).unwrap().check());
        assert_eq!(false, Fraction::new(3, 0, 5, 0).unwrap().check());
        assert_eq!(false, Fraction::new(1, 1, 2, 2).unwrap().check());
    }
}
