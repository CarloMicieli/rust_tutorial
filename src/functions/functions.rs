fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug, PartialEq)]
enum DivError {
    DividedByZero,
}

fn div(a: i32, b: i32) -> Result<i32, DivError> {
    match b {
        0 => Err(DivError::DividedByZero),
        _ => Ok(a / b),
    }
}

/// Computes the n-th fibonacci number (trivial implementation - very slow!!!)
pub fn fib(n: i32) -> i32 {
    match n {
        0 => panic!("n cannot be 0!"),
        1 => 1,
        2 => 1,
        i => fib(i - 1) + fib(i - 2),
    }
}

/// A generic implementation for the max function
fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_should_add_two_i32_values() {
        assert_eq!(sum(40, 2), 42);
    }

    #[test]
    fn max_should_find_max_between_two_i32_values() {
        let m = max(21, 42);
        assert_eq!(m, 42);
    }

    #[test]
    fn max_should_find_max_between_two_f32_values() {
        let m = max(21.0f32, 42.45f32);
        assert_eq!(m, 42.45f32);
    }

    #[test]
    fn div_should_return_an_ok_result_when_divisor_is_not_zero() {
        let res = div(84, 2);
        assert_eq!(res, Ok(42));
    }

    #[test]
    fn div_should_return_an_err_result_when_divisor_is_zero() {
        let res = div(84, 0);
        assert_eq!(res, Err(DivError::DividedByZero));
    }

    #[test]
    fn fib_should_compute_the_nth_fibonacci_number() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(10), 55);
    }
}
