use ascii::{self, AsciiString, IntoAsciiString};

pub fn lrc_hash_v1(target: &str, code_length: usize) -> AsciiString {
    let inner_data_length = code_length * 8;
    let binary_vector = decode2binary_vector(target.as_bytes());
    let padded_vector = padding_data_v1(binary_vector, inner_data_length);
    let calculated_vector = calculate_hash_v1(padded_vector, inner_data_length);

    if calculated_vector.len() != inner_data_length {
        println!("this result vector is ILLEGAL!!");
    }

    println!("this result vector is LEGALL!!!!!!");
    decode_data_v1(calculated_vector)
}
fn decode2binary_vector(orgin_data: &[u8]) -> Vec<u8> {
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

fn padding_data_v1(binary_vector: Vec<u8>, data_length: usize) -> Vec<u8> {
    print_vector(&binary_vector);
    let mut paded_vector = binary_vector;
    let remainder = paded_vector.len() % data_length;
    if remainder > 0 {
        let mut index = 0;
        while index < (data_length - remainder) {
            paded_vector.push(0);
            index += 1;
        }
    }

    print_vector(&paded_vector);

    if paded_vector.len() / data_length == 1 {
        let mut index = 0;
        while index < data_length {
            paded_vector.push(0);
            index += 1;
        }
    }

    print_vector(&paded_vector);
    paded_vector
}

fn calculate_hash_v1(paded_vector: Vec<u8>, data_length: usize) -> Vec<u8> {
    let mut len_time = 0;
    let mut calculate_vector: Vec<u8> = Vec::from(&paded_vector[..data_length]);
    println!("clone finished");
    print_vector(&calculate_vector);
    println!("\nstart process");
    while len_time < (paded_vector.len() / data_length) - 1 {
        //pent is process each number time
        let mut pent = 0;
        println!("start process innel");
        while pent < data_length {
            println!("now the first index {}", pent);
            println!("now the first result {}", calculate_vector[pent]);
            println!(
                "now the second index {}",
                (len_time + 1) * data_length + pent
            );
            println!(
                "now the second resutl {}",
                paded_vector[(len_time + 1) * data_length + pent]
            );
            let result_x =
                calculate_vector[pent] ^ paded_vector[(len_time + 1) * data_length + pent];
            println!("there are reuslt {}\n", result_x);
            calculate_vector[pent] = result_x;
            pent += 1;
        }
        len_time += 1;
    }
    println!("end process\n");
    calculate_vector
}

fn decode_data_v1(calculated_vector: Vec<u8>) -> AsciiString {
    let mut result_vector: Vec<u8> = Vec::new();
    let mut index = 0;
    while index < calculated_vector.len() / 8 {
        let mut bit_index = 0;
        let mut bit_result = 0;
        while bit_index < 8 {
            bit_result |= calculated_vector[8 * index + bit_index] << (7 - bit_index);
            bit_index += 1;
        }
        result_vector.push(bit_result);
        index += 1;
    }
    print!("the result_vector = ");
    print_vector(&result_vector);
    let result_data = match result_vector.into_ascii_string() {
        Ok(result) => result,
        Err(e) => {
            panic!("decoding error because: {}", e);
        }
    };
    println!("The Result = {}", result_data);
    result_data
}

fn print_vector(vector: &Vec<u8>) {
    for e in vector {
        print!("{}", *e);
    }
    println!();
}
