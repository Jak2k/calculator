fn main() {
    let args: Vec<String> = std::env::args().collect();
    // when no arguments are given, run the interactive mode
    if args.len() == 1 {
        calc::interactive();
        return;
    } else {
        // when arguments are given, run the non-interactive mode
        let mut input = String::new();
        for arg in args[1..].iter() {
            input.push_str(&arg);
            input.push(' ');
        }
        let input = input.replace('"', "").replace("\n", "");
        match calc::eval(&input) {
            Ok(n) => {
                println!("{}", n);
                return ();
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
