#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_create_an_empty_vector() {
        let v: Vec<u32> = Vec::new();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);
    }

    #[test]
    fn should_init_a_vector_with_the_same_value() {
        let v: Vec<u32> = vec![42u32; 6];
        assert_eq!(v.len(), 6);
        assert_eq!(v, [42, 42, 42, 42, 42, 42]);
    }

    #[test]
    fn should_create_an_empty_vector_with_an_initial_capacity() {
        let v: Vec<u32> = Vec::with_capacity(16);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 16);
    }

    #[test]
    fn should_create_a_new_vector_from_iterator_values() {
        let v: Vec<u32> = (1..5).collect();
        assert_eq!(v.len(), 4);
        assert_eq!(v, [1, 2, 3, 4]);
    }

    #[test]
    fn should_push_new_elements_onto_vectors() {
        let mut v: Vec<u32> = Vec::with_capacity(16);
        v.push(1);
        v.push(2);
        assert_eq!(v, [1, 2]);
    }

    #[test]
    fn should_pop_elements_from_a_vector() {
        let mut v: Vec<u32> = vec![1, 2, 3, 4];
        let el = v.pop();
        assert_eq!(el, Some(4));
        assert_eq!(v, [1, 2, 3]);
    }

    #[test]
    fn should_increase_a_vector_adding_elements_over_its_capacity() {
        let mut v: Vec<u32> = Vec::with_capacity(2);
        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.capacity(), 2);
        assert_eq!(v, [1, 2]);

        v.push(3);
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v.capacity(), 4);
        assert_eq!(v, [1, 2, 3, 4]);

        v.push(5);
        assert_eq!(v.len(), 5);
        assert_eq!(v.capacity(), 8);
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_insert_an_element_at_given_index() {
        let mut v: Vec<u32> = vec![10, 20, 30, 40, 50];
        v.insert(2, 99);
        assert_eq!(v.len(), 6);
        assert_eq!(v, [10, 20, 99, 30, 40, 50]);
    }

    #[test]
    fn should_remove_an_element_at_given_index() {
        let mut v: Vec<u32> = vec![10, 20, 30, 40, 50];
        let removed = v.remove(2);

        assert_eq!(removed, 30);
        assert_eq!(v.len(), 4);
        assert_eq!(v, [10, 20, 40, 50]);
    }
}
