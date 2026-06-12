fn main() {
    let case_simple = "Hello World";
    let case_distribution1 = "test1";
    let case_distribution2 = "test2";
    let case_long_text = "Slices let you reference a contiguous sequence of elements in a collection. \
        A slice is a kind of reference, so it does not have ownership. \
        Here’s a small programming problem: \
        Write a function that takes a string of words separated by spaces and \
        returns the first word it finds in that string. \
        If the function doesn’t find a space in the string, \
        the whole string must be one word, so the entire string should be returned.";
    let case_with_utf8_1 = "你好";

    // test simple case
    let processed_case_simple = lrc_hash_v1(case_simple, 128);
    println!("Test simple funtion result: {}", processed_case_simple);

    // test distribution case
    // let processed_case_distribution1 = hash_process_simple(case_distribution1.as_bytes());
    // let processed_case_distribution2 = hash_process_simple(case_distribution2.as_bytes());
    println!("Test simple function distribution result:");
    // println!("processed_case_distribution1 = {processed_case_distribution1}");
    // println!("processed_case_distribution2 = {processed_case_distribution2}");

    // test long test case
    let processed_case_long_text = lrc_hash_v1(case_long_text, 128);
    println!(
        "Test simple function long text result: {}",
        processed_case_long_text
    );
}

fn lrc_hash_v1(target: &str, code_length: usize) -> String {
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
    let remainder = temp_vector.len() % code_length;
    if remainder > 0 {
        let mut index = 0;
        while index < (code_length - remainder) {
            temp_vector.push(0);
            index += 1;
        }
    }
    let test_vector_a = temp_vector.clone();

    print_vector(&temp_vector);

    if temp_vector.len() / code_length == 1 {
        let mut index = 0;
        while index < code_length {
            temp_vector.push(0);
            index += 1;
        }
    }

    print_vector(&temp_vector);

    // TODO: the process method mite be error
    let mut len_time = 0;
    let mut result_vector: Vec<u8> = Vec::from(&temp_vector[..code_length]);
    println!("clone finished");
    print_vector(&result_vector);
    println!("\nstart process");
    while len_time < (temp_vector.len() / code_length) - 1 {
        //pent is process each number time
        let mut pent = 0;
        println!("start process innel");
        while pent < code_length {
            println!("now the first index {}", pent);
            println!("now the first result {}", result_vector[pent]);
            println!("now the second index {}", len_time * code_length + pent);
            println!(
                "now the second resutl {}",
                temp_vector[(len_time + 1) * code_length + pent]
            );
            let result_x = result_vector[pent] ^ temp_vector[(len_time + 1) * code_length + pent];
            println!("there are reuslt {}\n", result_x);
            result_vector[pent] = result_x;
            pent += 1;
        }
        len_time += 1;
    }
    println!("end process\n");
    println!(
        "are they same ? a: {}",
        jugetment_same(&test_vector_a, &result_vector)
    );

    if result_vector.len() != code_length {
        println!("this result vector is ILLEGAL!!");
    }
    println!("this result vector is LEGALL!!!!!!");
    let result = match String::from_utf8(result_vector) {
        Ok(string) => string,
        Err(e) => {
            panic!("decoding error becose: {}", e);
        }
    };
    println!("The Result = {}", result);
    result
}

fn print_vector(vector: &Vec<u8>) {
    for e in vector {
        print!("{}", *e);
    }
    println!();
}

fn jugetment_same(vector_a: &Vec<u8>, vector_b: &Vec<u8>) -> bool {
    let mut index = 0;
    while index < vector_a.len() {
        if vector_a[index] != vector_b[index] {
            return false;
        }
        index += 1;
    }
    true
}
