use std::collections::binary_heap;

use ascii::{self, AsciiString, IntoAsciiString};

fn main() {
    let case_simple = "Hello World";
    // let case_distribution1 = "test1";
    // let case_distribution2 = "test2";
    /*
    * let case_long_text = "Slices let you reference a contiguous sequence of elements in a collection. \
        A slice is a kind of reference, so it does not have ownership. \
        Here’s a small programming problem: \
        Write a function that takes a string of words separated by spaces and \
        returns the first word it finds in that string. \
        If the function doesn’t find a space in the string, \
        the whole string must be one word, so the entire string should be returned.";
    */
    let mid_long_test = "Slices let you reference a contiguous sequence of elements in a collection. \
        A slice is a kind of reference, so it does not have ownership. \
        Here’s a small programming problem: \
        Write a function that takes a string of words separated by spaces and \
        returns the first word it finds in that string.";

    // let case_with_utf8_1 = "你好";

    // test simple case
    let processed_case_simple = lrc_hash_v1(case_simple, 128);
    println!("Test simple funtion result: {}", processed_case_simple);

    // test distribution case
    // let processed_case_distribution1 = hash_process_simple(case_distribution1.as_bytes());
    // let processed_case_distribution2 = hash_process_simple(case_distribution2.as_bytes());
    // println!("Test simple function distribution result:");
    // println!("processed_case_distribution1 = {processed_case_distribution1}");
    // println!("processed_case_distribution2 = {processed_case_distribution2}");

    // test long test case
    // let processed_case_long_text = lrc_hash_v1(case_long_text, 128);
    // println!(
    //     "Test simple function long text result: {}",
    //     processed_case_long_text
    // );

    // test mid test case
    let processed_case_mid_long_test = lrc_hash_v1(mid_long_test, 128);
    println!(
        "Test simple function mid long text result : {}",
        processed_case_mid_long_test
    );
}

fn lrc_hash_v1(target: &str, code_length: usize) -> AsciiString {
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

    /*
    * let result_data = match String::from_utf8(result_vector) {
        Ok(string) => string,
        Err(e) => {
            panic!("decoding error becose: {}", e);
        }
    };
    */
    // let result_data = match result_vector.into_ascii_string_un() {
    //     Ok(result) => result,
    //     Err(e) => {
    //         panic!("decoding error because: {}", e);
    //     }
    // };
    let result_data = unsafe { result_vector.into_ascii_string_unchecked() };
    println!("The Result = {}", result_data);
    result_data
}

fn print_vector(vector: &Vec<u8>) {
    for e in vector {
        print!("{}", *e);
    }
    println!();
}
