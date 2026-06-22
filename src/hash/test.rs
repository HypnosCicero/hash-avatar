#[cfg(test)]
mod hash_algorithm_tests {
    use crate::hash;

    #[test]
    fn test_decode2binary_vector() {
        let binary_vector = hash::algorithm::decode2binary_vector("123".as_bytes());
        assert!(judge_u8_vector_is_binary(&binary_vector));
    }

    fn judge_u8_vector_is_binary(vector: &Vec<u8>) -> bool {
        let mut result = true;
        for elemnt in vector {
            if *elemnt > 1 {
                result = false;
                break;
            }
        }
        result
    }

    #[test]
    fn test_padding_data_v1() {
        let mut test_vector = vec![1, 2, 3];
        let origin_vector_length = test_vector.len();
        let standard_of_data_length = 128 * 8;
        hash::algorithm::padding_data_v1(&mut test_vector, standard_of_data_length);
        if origin_vector_length < standard_of_data_length {
            assert_eq!(test_vector.len(), standard_of_data_length * 2);
        }
        assert!(test_vector.len() % standard_of_data_length == 0);
    }
}
