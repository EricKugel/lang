mod token;

use std::fs;
use token::Token;

const valid_numbers: &str = "1234567890.";
const valid_characters: &str = "+-/*^";
const pemdas: [&str; 3] = ["^", "*/", "+-"];

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

fn parse_code(code: &str) -> Vec<Token> {
    let code = format(code);
    let mut object = vec!();
    let mut i = 0;

    while i < code.len() {
        let mut c = &code[i..i + 1];
        if c == "(" {
            let block = get_block_at(i, &code[..]);
            i += block.len();
            object.push(Token::Block(parse_code(&block[1..block.len() - 1])));
        } else {
            let mut token_str = String::new();
            if valid_numbers.contains(c) {
                while valid_numbers.contains(c) && i < code.len() {
                    token_str += c;
                    i += 1;
                    // may cause issues
                    c = &code[i..i + 1];
                }
                object.push(Token::Number(token_str.to_string().parse().unwrap()));
            } else if valid_characters.contains(c) {
                object.push(Token::Operator(c.to_string()));
                i += 1;
            }
        }
    }
    object
}

fn format(code: &str) -> String {
    let mut new_code = String::new();
    for (i, c) in code.chars().enumerate() {
        if c == '(' && new_code.len() > 0 && valid_numbers.contains(&code[i-1..i])  {
            new_code.push('*');
        }
        if valid_numbers.contains(c) || valid_characters.contains(c) || c == '(' || c == ')' {
            new_code.push(c);
        }
    }
    new_code
}

fn get_block_at(i: usize, code: &str) -> &str {
    let mut close_index = i + 1;
    let mut counter = 1;
    while counter != 0 && close_index < code.len() {
        counter += match &code[close_index..close_index + 1] {
            "(" => 1,
            ")" => -1,
            _ => 0
        };
        close_index += 1;
    }
    &code[i..close_index]
}

fn solve(tokens: &mut Vec<Token>) -> f64 {
    for i in 0..tokens.len() {
        if let Token::Block(child_tokens) = &tokens[i] {
            tokens[i] = Token::Number(solve(&mut child_tokens));
        }
    }
    unimplemented!();
}