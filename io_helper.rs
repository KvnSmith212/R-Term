use std::io;

pub fn handle_read() -> String {
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
pub fn split_line(line : &str) -> Vec<String> {
    let split_line = line.trim().split(" ");
    let temp_args: Vec<&str> = split_line.collect();
    let mut args = Vec::new();
    
    for arg in temp_args {
        args.push(arg.to_string());
    }

    args
}