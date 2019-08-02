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

        let (n, d) = Rational::reduce_values(num, den);
        Rational { n, d }
    }

    fn reduce_values(a: i32, b: i32) -> (i32, i32) {
        let sign = if a < 0 || b < 0 { -1 } else { 1 };
        let div = gcd(a.abs(), b.abs());
        (sign * (a / div).abs(), (b / div).abs())
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

    pub fn is_positive(&self) -> bool {
        !Rational::is_negative(self)
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
        match self {
            Rational { n, d: 1 } => write!(f, "{}", n),
            Rational { n, d } if *n < 0 || *d < 0 => write!(f, "-{}/{}", n.abs(), d.abs()),
            Rational { n, d } => write!(f, "{}/{}", n, d),
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

impl ops::Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        let Rational { n: num, d: den } = self;
        if self.is_positive() {
            Rational::new(-num, den)
        } else {
            Rational::new(num.abs(), den.abs())
        }
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

impl ops::SubAssign for Rational {
    fn sub_assign(&mut self, other: Self) {
        use std::ops::Sub;
        *self = self.sub(other);
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

impl ops::AddAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        use std::ops::Add;
        *self = self.add(other);
    }
}

#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_create_rational_numbers_from_whole_numbers() {
        let five = Rational::from(5);
        assert_eq!(five.numerator(), 5);
        assert_eq!(five.denominator(), 1);
    }

    #[test]
    fn should_negate_rational_numbers() {
        let one_half = Rational::new(1, 2);
        let minus_one_half = Rational::new(-1, -2);

        assert_eq!(-one_half, minus_one_half);
        assert_eq!(-minus_one_half, one_half);
    }

    #[test]
    fn should_display_rational_numbers() {
        let one_half = Rational::new(1, 2);
        let one = Rational::from(1);
        assert_eq!(one_half.to_string(), "1/2");
        assert_eq!(one.to_string(), "1");
    }

    #[test]
    fn should_display_negative_rational_numbers() {
        let r1 = Rational::new(-1, 2);
        let r2 = Rational::new(-1, -2);
        let r3 = Rational::new(1, -2);

        assert_eq!(r1.to_string(), "-1/2");
        assert_eq!(r2.to_string(), "-1/2");
        assert_eq!(r3.to_string(), "-1/2");
    }

    #[test]
    fn should_sum_two_rational_numbers() {
        let one_third = Rational::new(1, 3);
        let one_half = Rational::new(1, 2);

        let sum = one_half + one_third;
        assert_eq!(sum, Rational::new(5, 6));
    }

    #[test]
    fn should_sum_and_assign_two_rational_numbers() {
        let one_half = Rational::new(1, 2);
        let mut r = one_half;

        r += one_half;

        assert_eq!(r, Rational::from(1));
    }

    #[test]
    fn should_substract_and_assign_two_rational_numbers() {
        let one_half = Rational::new(1, 2);
        let mut r = Rational::from(1);

        r -= one_half;

        assert_eq!(r, one_half);
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
