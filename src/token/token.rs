use super::operator::Operator;

pub enum Token {
    Operator(Operator),
    Number(f64),
    Block(Vec<Token>)
}