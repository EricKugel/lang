mod token;

use std::fs;
use token::token::Token;
use token::operator::Operator;

const valid_numbers: &str = "1234567890.";
const valid_characters: &str = "+-/*^";
const pemdas: [&str; 3] = ["^", "*/", "+-"];

fn main() {
    let code = fs::read_to_string("C:/Files/Rust/lang/main.lang").unwrap();
    println!("{}", solve(&mut parse_code(&code)));
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
                object.push(Token::Operator(Operator::new(c)));
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
    for pemdas_index in 0..pemdas.len() {
        let current_op = pemdas[pemdas_index];
        let mut i = 0;
        while i < tokens.len() - 2 {
            if let Token::Operator(op) = &tokens[i + 1] {
                if current_op.contains(&op.to_string()) {
                    let mut first: &f64 = &0.0;
                    let mut second: &f64 = &0.0;

                    if let Token::Block(block) = &tokens[i] {
                        tokens[i] = Token::Number(solve(&mut block));
                    } if let Token::Block(block) = &tokens[i + 2] {
                        tokens[i + 2] = Token::Number(solve(&mut block));
                    }

                    if let Token::Number(num) = &tokens[i] {
                        first = num;
                    } if let Token::Number(num) = &tokens[i + 2] {
                        second = num;
                    }
                    
                    let result = match op {
                        Operator::Add => first + second,
                        Operator::Subtract => first - second,
                        Operator::Multiply => first * second,
                        Operator::Divide => first / second,
                        Operator::Exponent => first.powf(*second),
                    };

                    tokens = vec!(tokens[0..i], vec!(Token::Number(result))[0..1], tokens[i + 3..]);
                    i -= 2;
                }
            }
            i += 2;
        }
    }
    if let Token::Number(num) = tokens[0] {
        return num;
    } else {
        return 0.0;
    }
}