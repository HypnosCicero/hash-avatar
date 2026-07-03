use ascii::{self, AsciiString, IntoAsciiString};

pub fn lrc_hash_v1(target: &str, code_length: usize) -> AsciiString {
    let inner_data_length = code_length * 8;
    let mut binary_vector = decode2binary_vector(target.as_bytes());
    padding_data_v1(&mut binary_vector, inner_data_length);
    let calculated_vector = calculate_hash_v1(binary_vector, inner_data_length);
    decode_data_v1(calculated_vector)
}

pub(crate) fn decode2binary_vector(orgin_data: &[u8]) -> Vec<u8> {
    let mut binary_vector: Vec<u8> = Vec::new();
    for temp_data in orgin_data {
        let mut i = 7;
        while i >= 0 {
            let data = *temp_data >> i & 1;
            binary_vector.push(data);
            i -= 1;
        }
    }
    binary_vector
}

pub(crate) fn padding_data_v1(pading_vector: &mut Vec<u8>, data_length: usize) {
    let remainder = pading_vector.len() % data_length;
    if remainder > 0 {
        let mut index = 0;
        while index < (data_length - remainder) {
            pading_vector.push(0);
            index += 1;
        }
    }

    if pading_vector.len() / data_length == 1 {
        let mut index = 0;
        while index < data_length {
            pading_vector.push(0);
            index += 1;
        }
    }
}

pub(crate) fn calculate_hash_v1(paded_vector: Vec<u8>, data_length: usize) -> Vec<u8> {
    let mut len_time = 0;
    let mut calculate_vector: Vec<u8> = Vec::from(&paded_vector[..data_length]);
    while len_time < (paded_vector.len() / data_length) - 1 {
        let mut pent = 0;
        while pent < data_length {
            let result_x =
                calculate_vector[pent] ^ paded_vector[(len_time + 1) * data_length + pent];
            calculate_vector[pent] = result_x;
            pent += 1;
        }
        len_time += 1;
    }
    calculate_vector
}

pub(crate) fn decode_data_v1(calculated_vector: Vec<u8>) -> AsciiString {
    let mut result_vector: Vec<u8> = Vec::new();
    let mut index = 0;
    while index < calculated_vector.len() / 8 {
        let mut bit_index = 0;
        let mut bit_result = 0;
        while bit_index < 8 {
            bit_result |= calculated_vector[8 * index + bit_index] << (7 - bit_index);
            bit_index += 1;
        }
        result_vector.push(bit_result % 127);
        index += 1;
    }
    let result_data = match result_vector.into_ascii_string() {
        Ok(result) => result,
        Err(e) => {
            panic!("decoding error because: {}", e);
        }
    };
    result_data
}
