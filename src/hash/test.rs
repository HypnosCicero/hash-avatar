#[cfg(test)]
mod hash_algorithm_tests {
    use crate::hash;

    #[test]
    fn test_decode2binary_vector() {
        let binary_vector = hash::algorithm::decode2binary_vector("123".as_bytes());
        assert!(juede_u8_vector_is_binary(binary_vector));
    }

    fn juede_u8_vector_is_binary(vector: Vec<u8>) -> bool {
        let mut result = true;
        for elemnt in vector {
            if elemnt > 1 {
                result = false;
            }
        }
        result
    }
}
