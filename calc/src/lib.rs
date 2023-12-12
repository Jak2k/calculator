pub mod parser;
pub mod runtime;
pub mod tokenizer;

pub fn main(input: &str) -> Result<(), &'static str> {
    let tokens: Vec<tokenizer::Token> = tokenizer::tokenize(input)?;

    println!("Tokens: {:?}", tokens);

    let ast = parser::parse(tokens)?;

    println!("AST: {:#?}", ast);

    let result = runtime::eval(ast);

    println!("Result: {}", result);

    Ok(())
}
