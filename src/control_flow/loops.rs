/// Sums numbers from 'from' to 'to' with a while
pub fn sum(from: i32, to: i32) -> i32 {
    let mut i = from;
    let mut sum = 0;

    while i < to {
        sum += i;
        i += 1;
    }

    sum
}

/// Sums numbers from 'from' to 'to' with a for
pub fn sum2(from: i32, to: i32) -> i32 {
    let mut sum = 0;
    for i in from..to {
        sum += i;
    }

    sum
}

pub fn sum3(from: i32, to: i32) -> i32 {
    let mut i = from;
    let mut sum = 0;

    loop {
        if i == to {
            break;
        }

        sum += i;
        i += 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_make_the_sum_from_a_number_to_another() {
        let s = sum(1, 10);
        assert_eq!(45, s);
    }

    #[test]
    fn should_make_the_sum_from_a_number_to_another2() {
        let s = sum2(1, 10);
        assert_eq!(45, s);
    }

    #[test]
    fn should_make_the_sum_from_a_number_to_another3() {
        let s = sum3(1, 10);
        assert_eq!(45, s);
    }
}
