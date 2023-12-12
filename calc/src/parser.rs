use crate::tokenizer::Token;

#[derive(Debug)]
pub enum AstNode {
    Add(Box<AstNode>, Box<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),
    Mul(Box<AstNode>, Box<AstNode>),
    Div(Box<AstNode>, Box<AstNode>),
    Num(f64),
}

pub fn parse(tokens: Vec<Token>) -> Result<AstNode, &'static str> {
    // This parses the tokens into an AST recursively.

    let mut tokens = tokens.into_iter().peekable();

    // when tokens is empty, return error
    if tokens.peek().is_none() {
        return Err("No tokens");
    }

    // when tokens is lenght 1, it must be a number
    if tokens.len() == 1 {
        match tokens.next().unwrap() {
            Token::Number(n) => return Ok(AstNode::Num(n)),
            _ => return Err("Expcted a number"),
        }
    }

    // when tokens is longer than 2, call this function recursively
    if tokens.len() > 2 {
        let left = parse(vec![tokens.next().unwrap()])?;
        let op = tokens.next().unwrap();
        // all other tokens are on the right side
        let right = parse(tokens.collect::<Vec<Token>>())?;

        match op {
            Token::Operator(c) => {
                match c {
                    '+' => return Ok(AstNode::Add(Box::new(left), Box::new(right))),
                    '-' => return Ok(AstNode::Sub(Box::new(left), Box::new(right))),
                    '*' => return Ok(AstNode::Mul(Box::new(left), Box::new(right))),
                    '/' => {
                        // when it's a division, check if the right side is not 0
                        if let AstNode::Num(n) = right {
                            if n == 0.0 {
                                return Err("Division by zero");
                            }
                        };
                        return Ok(AstNode::Div(Box::new(left), Box::new(right)));
                    }
                    _ => return Err("Invalid operator"),
                }
            }
            _ => return Err("Expected an operator"),
        }
    };

    Err("Invalid expression")
}
