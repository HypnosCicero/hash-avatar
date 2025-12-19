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
    let processed_case_simple = hash_process_simple(case_simple.as_bytes());
    println!("Test simple funtion result:");
    println!("processed_case_simple = {processed_case_simple}\n");

    // test distribution case
    let processed_case_distribution1 = hash_process_simple(case_distribution1.as_bytes());
    let processed_case_distribution2 = hash_process_simple(case_distribution2.as_bytes());
    println!("Test simple function distribution result:");
    println!("processed_case_distribution1 = {processed_case_distribution1}");
    println!("processed_case_distribution2 = {processed_case_distribution2}");

    // test long test case
    let processed_case_long_text = hash_process_simple(case_long_text.as_bytes());
    println!("Test simple function long text result:");
    println!("processed_case_long_text = {processed_case_long_text}");
}

fn hash_process_simple(target: &[u8]) -> String {
    let group_number = 255;
    return String::from("");
}
