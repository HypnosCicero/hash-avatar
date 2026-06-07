use std::io::{Error, Read};

fn main() {
    let case_simple: &str = "Hello World";
    let case_distribution1 = "test1";
    let case_distribution2 = "test2";
    let case_long_text = "Slices let you reference a contiguous sequence of elements in a collection. \
        A slice is a kind of reference, so it does not have ownership. \
        Here’s a small programming problem: \
        Write a function that takes a string of words separated by spaces and \
        returns the first word it finds in that string. \
        If the function doesn’t find a space in the string, \
        the whole string must be one word, so the entire string should be returned.";

    // test simple case
    let processed_case_simple = lrc_hash_v1(case_simple);
    println!("Test simple funtion result:");

    // test distribution case
    // let processed_case_distribution1 = hash_process_simple(case_distribution1.as_bytes());
    // let processed_case_distribution2 = hash_process_simple(case_distribution2.as_bytes());
    println!("Test simple function distribution result:");
    // println!("processed_case_distribution1 = {processed_case_distribution1}");
    // println!("processed_case_distribution2 = {processed_case_distribution2}");

    // test long test case
    // let processed_case_long_text = hash_process_simple(case_long_text.as_bytes());
    println!("Test simple function long text result:");
    // println!("processed_case_long_text = {processed_case_long_text}");
}

fn lrc_hash_v1(target: &str) {
    let orgin_data = target.as_bytes();
    let mut temp_vector: Vec<u8> = Vec::new();
    for temp_data in orgin_data {
        let mut i = 7;
        while i >= 0 {
            // check out the
            let data = *temp_data >> i & 1;
            temp_vector.push(data);
            i -= 1;
        }
    }
    print_vector(&temp_vector);
    let remainder = temp_vector.len() % 7;
    if remainder > 0 {
        let mut index = 0;
        while index < (7 - remainder) {
            temp_vector.push(0);
            index += 1;
        }
    }
    print_vector(&temp_vector);
}

fn print_vector(vector: &Vec<u8>) {
    for e in vector {
        print!("{}", *e);
    }
    println!();
}
