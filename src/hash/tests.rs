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
    // preparation stage
    let mut test_vector = vec![1, 2, 3];
    let origin_vector_length = test_vector.len();
    let standard_of_data_length = 128 * 8;

    // test padding data
    hash::algorithm::padding_data_v1(&mut test_vector, standard_of_data_length);

    assert!(test_vector.len() % standard_of_data_length == 0);
    if origin_vector_length < standard_of_data_length {
        assert_eq!(test_vector.len(), standard_of_data_length * 2);
    }
}

#[test]
fn test_calculate_hash_v1() {
    // preparation stage
    let mut test_vector1: Vec<u8> = Vec::new();
    let mut test_vector2: Vec<u8> = Vec::new();
    let mut result_vector1: Vec<u8> = Vec::new();
    let mut result_vector2: Vec<u8> = Vec::new();
    let standard_of_data_length = 128 * 8;
    prepare_test_vectors(
        &mut test_vector1,
        &mut test_vector2,
        standard_of_data_length,
    );
    prepare_result_vectors(
        &mut result_vector1,
        &mut result_vector2,
        standard_of_data_length,
    );

    // test calculate hash algorithm
    let un_test_result_vector1 =
        hash::algorithm::calculate_hash_v1(test_vector1, standard_of_data_length);
    let un_test_result_vector2 =
        hash::algorithm::calculate_hash_v1(test_vector2, standard_of_data_length);

    assert_eq!(un_test_result_vector1.len(), standard_of_data_length);
    assert_eq!(un_test_result_vector2.len(), standard_of_data_length);

    assert!(un_test_result_vector1 == result_vector1);
    assert!(un_test_result_vector2 == result_vector2);
}

fn prepare_test_vectors(vector1: &mut Vec<u8>, vector2: &mut Vec<u8>, length: usize) {
    push_binary_vector(vector1, length * 2, 1, 0, 3);
    push_binary_vector(vector2, length * 2, 1, 0, length - 1);
}

fn prepare_result_vectors(
    result_vector1: &mut Vec<u8>,
    result_vector2: &mut Vec<u8>,
    length: usize,
) {
    let time_length = length;

    push_binary_vector(result_vector1, time_length, 1, 0, 3);
    push_binary_vector(result_vector2, time_length, 1, 0, time_length - 1);
}

fn push_binary_vector(
    target_vector: &mut Vec<u8>,
    around: usize,
    pushdata1: u8,
    pushdata2: u8,
    conditon1: usize,
) {
    let mut index = 0;
    while index < around {
        if index < conditon1 {
            target_vector.push(pushdata1);
        } else {
            target_vector.push(pushdata2);
        }
        index += 1;
    }
}
