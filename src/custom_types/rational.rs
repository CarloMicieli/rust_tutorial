use std::cmp;
use std::fmt;
use std::ops;

///A rational number representation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rational {
    n: i32,
    d: i32,
}

impl Rational {
    pub fn new(num: i32, den: i32) -> Rational {
        assert!(den != 0, "Denominator is zero!");
        let div = gcd(num.abs(), den.abs());
        Rational {
            n: num / div,
            d: den / div,
        }
    }

    pub fn numerator(&self) -> i32 {
        self.n
    }

    pub fn denominator(&self) -> i32 {
        self.d
    }

    /// A new rational number
    pub fn from(num: i32) -> Rational {
        Rational { n: num, d: 1 }
    }

    pub fn is_negative(&self) -> bool {
        self.n < 0 || self.d < 0
    }
}

// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.d {
            1 => write!(f, "{}", self.n),
            _ => write!(f, "{}/{}", self.n, self.d),
        }
    }
}

impl cmp::PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Rational {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let Rational { n: a, d: b } = self;
        let Rational { n: c, d: d } = other;
        (a * d).cmp(&(c * b))
    }
}

impl ops::Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let Rational { n: n1, d: d1 } = self;
        let Rational { n: n2, d: d2 } = other;
        Rational::new((n1 * d2) - (d1 * n2), d1 * d2)
    }
}

impl ops::Div for Rational {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let Rational { n: n1, d: d1 } = self;
        let Rational { n: n2, d: d2 } = other;

        if n2 == 0 {
            panic!("RHS numerator is zero!");
        }

        Rational::new(n1 * d2, n2 * d1)
    }
}

impl ops::Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let Rational { n: n1, d: d1 } = self;
        let Rational { n: n2, d: d2 } = other;
        Rational::new(n1 * n2, d1 * d2)
    }
}

impl ops::Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Rational { n: n1, d: d1 } = self;
        let Rational { n: n2, d: d2 } = other;
        Rational::new((n1 * d2) + (d1 * n2), d1 * d2)
    }
}

#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_sum_two_rational_numbers() {
        let one_third = Rational::new(1, 3);
        let one_half = Rational::new(1, 2);

        let sum = one_half + one_third;
        assert_eq!(sum, Rational::new(5, 6));
    }

    #[test]
    fn should_create_new_rational_numbers_simplifing_its_values() {
        let r = Rational::new(4, 6);
        assert_eq!(r, Rational::new(2, 3));
    }

    #[test]
    fn should_create_new_rational_numbers() {
        let r = Rational::new(2, 3);
        assert_eq!(r.to_string(), "2/3".to_string());
    }

    #[test]
    fn should_create_rational_numbers_from_number() {
        let r = Rational::from(2);
        assert_eq!(r.to_string(), "2".to_string());
    }

    #[test]
    fn should_check_whether_the_rational_number_is_positive() {
        let r = Rational::new(1, 3);
        assert!(r.is_negative() == false);
    }

    #[test]
    fn should_check_whether_the_rational_number_is_negative() {
        let r1 = Rational::new(1, -3);
        let r2 = Rational::new(-1, 3);
        let r3 = Rational::new(-1, -3);
        assert!(r1.is_negative());
        assert!(r2.is_negative());
        assert!(r3.is_negative());
    }

    #[test]
    fn should_multiply_two_rational_numbers() {
        let a = Rational::new(2, 3);
        let b = Rational::new(3, 4);

        assert_eq!(a * b, Rational::new(1, 2));
    }

    #[test]
    fn should_divide_two_rational_numbers() {
        let a = Rational::new(1, 2);
        let b = Rational::new(3, 4);

        assert_eq!(a / b, Rational::new(2, 3));
    }

    #[test]
    #[should_panic(expected = "RHS numerator is zero!")]
    fn should_panic_when_the_rhs_numerator_is_zero() {
        let a = Rational::new(1, 2);
        let b = Rational::new(0, 4);

        let _ = a / b;
    }

    #[test]
    fn should_make_the_difference_between_two_rational_numbers() {
        let a = Rational::new(2, 3);
        let b = Rational::new(3, 4);

        assert_eq!(a - b, Rational::new(-1, 12));
    }

    #[test]
    #[should_panic(expected = "Denominator is zero")]
    fn should_panic_when_denominator_is_zero() {
        let _ = Rational::new(1, 0);
    }

    #[test]
    fn should_compare_two_rational_numbers() {
        let one_third = Rational::new(1, 3);
        let one_half = Rational::new(1, 2);

        assert!(one_half == one_half);
        assert!(one_half > one_third);
        assert!(one_third < one_half);
    }
}
