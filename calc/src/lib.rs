pub mod parser;
pub mod runtime;
pub mod tokenizer;

fn output_interactive(input: &str) -> Result<(), &'static str> {
    let tokens: Vec<tokenizer::Token> = tokenizer::tokenize(input)?;

    println!("Tokens: {:?}", tokens);

    let ast = parser::parse(tokens)?;

    println!("AST: {:#?}", ast);

    let result = runtime::eval(ast);

    println!("Result: {}", result);

    Ok(())
}

fn get_input() -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("> ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    s
}

fn welcome() {
    println!("Welcome to jCalc, a fast calculator written in rust!");
    println!("Type 'exit' to exit.");
    println!("Type 'help' to get help.");
}

fn help() {
    println!("  jCalc is a fast calculator written in rust!");

    println!("  It supports the following operators:");
    println!("  + - * /");
    println!("  Supportes numbers are integers and floats.");
    println!("  Example: 1 + 2 * 3 / 4");
    println!("  Spaces are ignored.");
    println!("  Brackets will be supported in future versions.");

    println!("  Type 'exit' to exit.");
    println!("  Type 'help' to get help.");
}

pub fn interactive() {
    welcome();

    loop {
        let input = get_input();

        match input.trim() {
            "exit" => break,
            "help" => {
                help();
                continue;
            }
            "" => continue,
            "clear" => {
                // clear screen
                print!("{}[2J", 27 as char);
                // jump to screen start
                print!("{}[1;1H", 27 as char);
                continue;
            }
            _ => {
                let start_time = std::time::Instant::now();
                match output_interactive(input.trim()) {
                    Ok(_) => (),
                    Err(e) => println!("Error: {}", e),
                }
                let end_time = std::time::Instant::now();

                let delta = end_time - start_time;
                let delta_ms = delta.as_nanos() as f64 / 1_000_000.0;

                println!("Time: {}ms", delta_ms);
            }
        }
    }
}

pub fn eval(input: &str) -> Result<f64, &'static str> {
    let tokens: Vec<tokenizer::Token> = tokenizer::tokenize(input)?;

    let ast = parser::parse(tokens)?;

    let result = runtime::eval(ast);

    Ok(result)
}
