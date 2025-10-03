use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let function = &args[1];
    let target = &args[2];
    println!("function is = {function}");
    println!("target is = {target}");
}
