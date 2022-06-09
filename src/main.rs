use std::fs;

fn main() {
    let code = fs::read_to_string("C:/Files/Rust/lang/main.lang").unwrap();
    let a1: &str = &code[0..1];
    let a2: &str = &code[2..];
    let b1: i32 = a1.parse().unwrap();
    let b2: i32 = a2.parse().unwrap();
    let op: &str = &code[1..2];
    let c = match op {
        "+" => b1 + b2,
        "-" => b1 - b2,
        "*" => b1 * b2,
        "/" => b1 / b2,
        _ => 0
    };
    println!("{}", c);
}