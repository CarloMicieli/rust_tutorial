pub fn thruth(x: i32) -> bool {
    if x == 42 {
        true
    } else {
        false
    }
}

pub fn if_as_expression(x: i32) -> &'static str {
    let val = if x > 0 { "positive" } else { "negative" };
    val
}

#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_find_truth_in_42() {
        assert_eq!(thruth(42), true);
    }

    #[test]
    fn should_not_find_truth_in_666() {
        assert_eq!(thruth(666), false);
    }

    #[test]
    fn should_use_ifs_as_expressions() {
        let v = if_as_expression(32);
        assert_eq!(v, "positive");
    }
}
