mod hash;

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
    let processed_case_simple = hash::lrc_hash_v1(case_simple, 128);
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
    let processed_case_mid_long_test = hash::lrc_hash_v1(mid_long_test, 128);
    println!(
        "Test simple function mid long text result : {}",
        processed_case_mid_long_test
    );
}
