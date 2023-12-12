#[derive(Debug)]
pub enum Token {
    Number(f64),
    Operator(char),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let input = input.replace(" ", "");
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut num = c.to_string();
                while let Some('0'..='9' | '.') = chars.peek() {
                    num.push(chars.next().unwrap());
                }
                // try to parse number or return error
                let numbr = match num.parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => return Err("Invalid number"),
                };
                tokens.push(Token::Number(numbr));
            }
            '+' | '-' | '*' | '/' => tokens.push(Token::Operator(c)),
            '\n' => (),
            _ => return Err("Invalid character"),
        }
    }
    Ok(tokens)
}
