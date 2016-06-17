use std::io;

macro_rules! printfl {
        // need to read up on rust macros, have no idea what's going on with: ($($tt:tt)*)
        ($($tt:tt)*) => {{
        use std::io::Write;
        print!($($tt)*);
        ::std::io::stdout().flush().unwrap();
    }}
}

fn main() {
    
    //Load config files

    //Run command loop 
    run_loop();

    //Perform cleanup and shutdown
}

fn run_loop() {

    loop {
        printfl!("> ");

        let input : String = handle_read();
        let args : Vec<String> = split_args(&input);
        let success : bool = execute(args);

        if !success {
            break;
        }
    }
}

fn handle_read() -> String {
    match read_line() {
        Err(e) => panic!("{:?}", e),
        Ok(value) => value,
    }
}

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    try!(io::stdin().read_line(&mut input));
    Ok(input)
}

/*
    Split input into args with the form: command arg1 arg2 arg3...
    Quotes, multiple lines, etc all ignore for now.
*/
fn split_args(input : &str) -> Vec<String> {
    let split = input.split(" ");
    let temp_args: Vec<&str> = split.collect();
    let mut args = Vec::new();
    
    for arg in temp_args {
        args.push(arg.to_string());
    }

    args
}

fn execute(args : Vec<String>) -> bool {
    for arg in args {
        println!("args: {}", arg)
    }
    
    false
}