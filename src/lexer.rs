#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),

    Plus,
    Minus,
    Slash,
    Star,

    LParen,
    RParen,

    EOF,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            c if c.is_whitespace() => continue,

            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),

            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),

            '0'..='9' => {
                let mut number = c.to_string();
                let mut is_float = false;
                while let Some(&next) = chars.peek() {
                    if next.is_digit(10) {
                        number.push(chars.next().unwrap());
                    } else if next == '.' && !is_float {
                        is_float = true;
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let num_value: f64 = number.parse().expect("Failed to parse number");
                tokens.push(Token::Number(num_value));
            }

            _ => panic!("Unkown character: {}", c),
        }
    }

    tokens.push(Token::EOF);

    tokens
}
