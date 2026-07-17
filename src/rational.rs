//! Exact rational arithmetic for the LP relaxation.
use std::cmp::Ordering;
use std::ops::Neg;

#[derive(Clone, Copy, Debug)]
pub struct Rat {
    pub num: i128,
    pub den: i128, // always > 0, fraction in lowest terms
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

impl Rat {
    pub fn new(num: i128, den: i128) -> Self {
        if den == 0 {
            panic!("Rat: zero denominator");
        }
        let g = gcd(num, den);
        let mut num = num / g;
        let mut den = den / g;
        if den < 0 {
            num = -num;
            den = -den;
        }
        Rat { num, den }
    }
    pub fn from_i(x: i64) -> Self {
        Rat::new(x as i128, 1)
    }
    pub fn zero() -> Self {
        Rat { num: 0, den: 1 }
    }
    pub fn one() -> Self {
        Rat { num: 1, den: 1 }
    }
    pub fn add(self, o: Rat) -> Rat {
        Rat::new(self.num * o.den + o.num * self.den, self.den * o.den)
    }
    pub fn sub(self, o: Rat) -> Rat {
        Rat::new(self.num * o.den - o.num * self.den, self.den * o.den)
    }
    pub fn mul(self, o: Rat) -> Rat {
        Rat::new(self.num * o.num, self.den * o.den)
    }
    pub fn div(self, o: Rat) -> Rat {
        Rat::new(self.num * o.den, self.den * o.num)
    }
    pub fn is_zero(self) -> bool {
        self.num == 0
    }
    pub fn is_positive(self) -> bool {
        self.num > 0
    }
    pub fn is_negative(self) -> bool {
        self.num < 0
    }
    pub fn is_integer(self) -> bool {
        self.den == 1
    }
    /// Floor, assuming den > 0.
    pub fn floor(self) -> i128 {
        let q = self.num / self.den;
        let r = self.num % self.den;
        if r < 0 {
            q - 1
        } else {
            q
        }
    }
    /// Ceiling, assuming den > 0.
    pub fn ceil(self) -> i128 {
        let q = self.num / self.den;
        let r = self.num % self.den;
        if r > 0 {
            q + 1
        } else {
            q
        }
    }
    /// Fractional part in [0, 1) as a Rat (assuming den > 0).
    pub fn frac(self) -> Rat {
        let f = self.floor();
        Rat::new(self.num - f * self.den, self.den)
    }
}

impl PartialEq for Rat {
    fn eq(&self, o: &Rat) -> bool {
        self.num == o.num && self.den == o.den
    }
}

impl Eq for Rat {}

impl PartialOrd for Rat {
    fn partial_cmp(&self, o: &Rat) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl Ord for Rat {
    fn cmp(&self, o: &Rat) -> Ordering {
        // den > 0 for both
        (self.num * o.den).cmp(&(o.num * self.den))
    }
}

impl Neg for Rat {
    type Output = Rat;
    fn neg(self) -> Rat {
        Rat::new(-self.num, self.den)
    }
}
