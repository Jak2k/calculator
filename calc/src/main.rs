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

fn main() {
    welcome();

    loop {
        let input = get_input();

        if input.trim() == "exit" {
            break;
        }

        if input.trim() == "help" {
            help();
            continue;
        }

        if input.trim() == "" {
            continue;
        }

        if input.trim() == "clear" {
            // clear screen
            print!("{}[2J", 27 as char);
            // jump to screen start
            print!("{}[1;1H", 27 as char);
            continue;
        }

        match calc::main(&input) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }
    }
}
