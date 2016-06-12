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

        let input : io::Result<()> = read_line();
        let args : Vec<String> = split_args(input);
        let success : bool = execute(args);

        if !success {
            break;
        }
    }
}

fn read_line() -> io::Result<()> {
    let mut input = String::new();

    try!(io::stdin().read_line(&mut input));

    //can I get just the string to return. SHOULD I return just the string?
    Ok(())
}

fn split_args(input : io::Result<()>) -> Vec<String> {
    let args : Vec<String> = vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()];

    args
}

fn execute(args : Vec<String>) -> bool {
    false
}