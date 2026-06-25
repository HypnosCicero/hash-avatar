use crate::hash;

#[test]
fn test_decode2binary_vector() {
    let binary_vector = hash::algorithm::decode2binary_vector("123".as_bytes());
    assert!(judge_u8_vector_is_binary(&binary_vector));
    let stander_vector: Vec<u8> = vec![
        0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1,
    ];
    assert_eq!(binary_vector, stander_vector);
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
    assert!(check_padded_vector(&test_vector, origin_vector_length, 0))
}

fn check_padded_vector(target_vector: &Vec<u8>, start_index: usize, pad_data: u8) -> bool {
    let mut index = start_index;
    let mut result = true;
    while index < target_vector.len() {
        if (target_vector[index] != pad_data) {
            result = false;
            break;
        }
        index += 1
    }
    result
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

fn push_binary_vector<T: Copy>(
    target_vector: &mut Vec<T>,
    around: usize,
    pushdata1: T,
    pushdata2: T,
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

#[test]
fn test_decode_hello_rust() {
    let data = vec![
        0, 1, 0, 0, 1, 0, 0, 0, // 'H'
        0, 1, 1, 0, 0, 1, 0, 1, // 'e'
        0, 1, 1, 0, 1, 1, 0, 0, // 'l'
        0, 1, 1, 0, 1, 1, 0, 0, // 'l'
        0, 1, 1, 0, 1, 1, 1, 1, // 'o'
        0, 0, 1, 0, 1, 1, 0, 0, // ','
        0, 0, 1, 0, 0, 0, 0, 0, // ' '
        0, 1, 0, 1, 0, 0, 1, 0, // 'R'
        0, 1, 1, 1, 0, 1, 0, 1, // 'u'
        0, 1, 1, 1, 0, 0, 1, 1, // 's'
        0, 1, 1, 1, 0, 1, 0, 0, // 't'
        0, 0, 1, 0, 0, 0, 0, 1, // '!'
    ];

    let result = hash::algorithm::decode_data_v1(data);
    assert_eq!(result, "Hello, Rust!");
}

#[test]
fn test_decode_128_bytes_long_ascii() {
    let data = vec![
        0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1,
        0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0,
        0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1,
        1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0,
        0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1,
        0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0,
        1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1,
        1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0,
        0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0,
        1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1,
        1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1,
        0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0,
        0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1,
        1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1,
        0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0,
        1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1,
        0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1,
        0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0,
        1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1,
        0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0,
        0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1,
        1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1,
        1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1,
        0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1,
        0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0,
        0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0,
        0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1,
        1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1,
        0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1,
        1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1,
        0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1,
        1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0,
        0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1,
        0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0,
        1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0,
    ];
    let result = hash::algorithm::decode_data_v1(data);
    assert_eq!(
        result,
        "Rust_Engineering_Detail_Oriented_Code_ASCII_Test_Vector_128_Bytes_Long_Without_Any_NonASCII_Characters_Just_For_System_Validation"
    );
}

#[test]
fn test_decode_including_control_characters() {
    let data = vec![
        0, 0, 0, 0, 1, 0, 1, 0, // '\n' (10)
        0, 1, 0, 0, 0, 0, 0, 1, // 'A' (65)
        0, 1, 0, 1, 0, 0, 1, 1, // 'S' (83)
        0, 1, 0, 0, 0, 0, 1, 1, // 'C' (67)
        0, 1, 0, 0, 1, 0, 0, 1, // 'I' (73)
        0, 1, 0, 0, 1, 0, 0, 1, // 'I' (73)
        0, 1, 0, 1, 1, 1, 1, 1, // '_' (95)
        0, 1, 0, 1, 0, 1, 0, 0, // 'T' (84)
        0, 1, 1, 0, 0, 1, 0, 1, // 'e' (101)
        0, 1, 1, 1, 0, 0, 1, 1, // 's' (115)
        0, 1, 1, 1, 0, 1, 0, 0, // 't' (116)
        0, 0, 0, 0, 1, 1, 0, 1, // '\r' (13)
    ];
    let result = hash::algorithm::decode_data_v1(data);
    assert_eq!(result, "\nASCII_Test\r");
}
