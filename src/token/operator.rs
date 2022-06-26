pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent
}

impl Operator {
    pub fn to_string(&self) -> String {
        match self {
            Operator::Add => "+".to_string(),
            Operator::Subtract => "-".to_string(),
            Operator::Multiply => "*".to_string(),
            Operator::Divide => "/".to_string(),
            Operator::Exponent => "^".to_string()
        }
    }

    pub fn new(sign: &str) -> Self {
        match sign {
            "+" => Operator::Add,
             "-" => Operator::Subtract,
             "*" => Operator::Multiply,
             "/" => Operator::Divide,
             "^" => Operator::Exponent,
             _ => Operator::Add,
        }
    }
}