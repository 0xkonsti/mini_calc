use crate::lexer::Token;

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<Token> for Op {
    fn from(token: Token) -> Self {
        match token {
            Token::Plus => Op::Add,
            Token::Minus => Op::Sub,
            Token::Star => Op::Mul,
            Token::Slash => Op::Div,
            _ => panic!("Invalid operator token: {:?}", token),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),

    BinaryOp(Box<Expr>, Op, Box<Expr>),
}

pub fn parse(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = parse_factor(tokens);

    while let Some(Token::Plus) | Some(Token::Minus) = tokens.first() {
        let op = Op::from(tokens.remove(0));
        let rhs = parse_factor(tokens);
        lhs = Expr::BinaryOp(Box::new(lhs), op, Box::new(rhs));
    }

    lhs
}

fn parse_factor(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = parse_primary(tokens);

    while let Some(Token::Star) | Some(Token::Slash) = tokens.first() {
        let op = Op::from(tokens.remove(0));
        let rhs = parse_primary(tokens);
        lhs = Expr::BinaryOp(Box::new(lhs), op, Box::new(rhs));
    }

    lhs
}

fn parse_primary(tokens: &mut Vec<Token>) -> Expr {
    match tokens.remove(0) {
        Token::Number(n) => Expr::Number(n),
        Token::LParen => {
            let expr = parse(tokens);
            if let Some(Token::RParen) = tokens.first() {
                tokens.remove(0);
                expr
            } else {
                panic!("Expected closing parenthesis");
            }
        }
        _ => panic!("Unexpected token: {:?}", tokens.first()),
    }
}
