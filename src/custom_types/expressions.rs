use std::fmt;

#[derive(Debug, Clone)]
pub enum Exp {
    Val { n: i32 },
    Variable { name: String, exp: Box<Exp> },
    Sum { lhs: Box<Exp>, rhs: Box<Exp> },
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Val { n } => write!(f, "Val({})", n),
            Exp::Variable { name, exp: _ } => write!(f, "{}", name),
            Exp::Sum { lhs, rhs } => write!(f, "({}) + ({})", lhs, rhs),
        }
    }
}

impl Exp {
    /// Returns a new constant value
    pub fn constant(val: i32) -> Exp {
        Exp::Val { n: val }
    }

    /// Returns a new `variable` bound to an `Exp`.
    ///
    /// Move the argument `exp`.
    pub fn variable(name: &str, exp: Exp) -> Exp {
        Exp::Variable {
            name: name.to_string(),
            exp: Box::new(exp),
        }
    }

    pub fn sum(lhs: Exp, rhs: Exp) -> Exp {
        Exp::Sum {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn eval(exp: &Exp) -> i32 {
        match exp {
            Exp::Val { n } => *n,
            Exp::Variable { name: _, exp: e } => Exp::eval(&e),
            Exp::Sum { lhs, rhs } => Exp::eval(&lhs) + Exp::eval(&rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_create_new_constant_expressions() {
        let e = Exp::constant(42);
        assert_eq!(e.to_string(), "Val(42)");
    }

    #[test]
    fn should_create_new_variable_with_a_bound_expression() {
        let e = Exp::constant(42);
        let variable_exp = Exp::variable("name", e);
        assert_eq!(variable_exp.to_string(), "name");
    }

    #[test]
    fn should_create_a_sum_expression() {
        let lhs = Exp::constant(21);
        let rhs = Exp::constant(1);

        let sum_exp = Exp::sum(lhs, rhs);
        assert_eq!(sum_exp.to_string(), "(Val(21)) + (Val(1))");
    }

    #[test]
    fn should_evaluate_expressions() {
        let e1 = Exp::sum(Exp::constant(19), Exp::constant(2));
        let e2 = Exp::sum(e1.clone(), e1);

        assert_eq!(Exp::eval(&e2), 42);
    }
}
