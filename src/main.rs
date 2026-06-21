mod hash;

fn main() {
    // perpare the data
    let case_simple = "Hello World";
    let case_distribution1 = "test1";
    let case_distribution2 = "test2";
    let mid_long_test = "Slices let you reference a contiguous sequence of elements in a collection. \
        A slice is a kind of reference, so it does not have ownership. \
        Here’s a small programming problem: \
        Write a function that takes a string of words separated by spaces and \
        returns the first word it finds in that string.";
    let case_long_text = "Slices let you reference a contiguous sequence of elements in a collection. \
        A slice is a kind of reference, so it does not have ownership. \
        Here’s a small programming problem: \
        Write a function that takes a string of words separated by spaces and \
        returns the first word it finds in that string. \
        If the function doesn’t find a space in the string, \
        the whole string must be one word, so the entire string should be returned.";

    let case_with_utf8_1 = "你好";

    // test simple case
    println!("###################### TEST SIMPLE CASE ######################");
    let processed_case_simple = hash::algorithm::lrc_hash_v1(case_simple, 128);
    println!("Test simple funtion result: {processed_case_simple}");
    println!("###################### TEST SIMPLE CASE ######################\n");

    // test distribution case
    println!("###################### TEST DISTRIBUTION CASE ######################");
    let processed_case_distribution1 = hash::algorithm::lrc_hash_v1(case_distribution1, 128);
    let processed_case_distribution2 = hash::algorithm::lrc_hash_v1(case_distribution2, 128);
    println!("Test simple function distribution result:");
    println!("processed_case_distribution1 = {processed_case_distribution1}");
    println!("processed_case_distribution2 = {processed_case_distribution2}");
    println!("###################### TEST DISTRIBUTION CASE ######################\n");

    // test mid text case
    println!("###################### TEST MID TEXT CASE ######################");
    let processed_case_mid_long_test = hash::algorithm::lrc_hash_v1(mid_long_test, 128);
    println!("Test simple function mid long text result : {processed_case_mid_long_test}");
    println!("###################### TEST MID TEXT CASE ######################\n");

    // test long text case
    println!("###################### TEST LONG TEXT CASE ######################");
    let processed_case_long_text = hash::algorithm::lrc_hash_v1(case_long_text, 128);
    println!("Test simple function long text result: {processed_case_long_text}");
    println!("###################### TEST LONG TEXT CASE ######################\n");

    // test utf-8 case
    println!("###################### TEST UTF-8 TEXT CASE ######################");
    let processed_case_utf8_text = hash::algorithm::lrc_hash_v1(case_with_utf8_1, 128);
    println!("The simple function utf8 text result: {processed_case_utf8_text}");
    println!("###################### TEST UTF-8 TEXT CASE ######################");
}
