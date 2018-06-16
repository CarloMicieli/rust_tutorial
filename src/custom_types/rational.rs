use std::fmt;

///A rational number representation.
#[derive(Debug)]
pub struct Rational {
    n: i32,
    d: i32,
}

impl Rational {
    pub fn new(num: i32, den: i32) -> Rational {
        Rational { n: num, d: den }
    }

    /// A new rational number
    pub fn from(num: i32) -> Rational {
        Rational { n: num, d: 1 }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.d {
            1 => write!(f, "{}", self.n),
            _ => write!(f, "{}/{}", self.n, self.d),
        }
    }
}

#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

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
}
