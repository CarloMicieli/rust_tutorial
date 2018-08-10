#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_create_an_empty_array() {
        const SIZE: usize = 6;

        let array: [u32; SIZE] = [0u32; SIZE];
        assert_eq!(array.len(), SIZE);
        assert_eq!(array[0], 0u32);
        assert_eq!(array[5], 0u32);
    }

}
