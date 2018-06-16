/// Silly function to print a greeting message
fn hello(name: &str) -> String {
    format!("Hello, {name}", name = name)
}

#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn hello_should_make_a_greeting() {
        assert_eq!(hello("rust"), "Hello, rust".to_string());
    }
}
