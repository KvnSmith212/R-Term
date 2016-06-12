fn main() {
    
    //Load config files

    //Run command loop 
    run_loop();

    //Perform cleanup and shutdown
}

fn run_loop() {

    loop {
        println!("> ");
        let input : String = read_line();
        let args : Vec<String> = split_args(input);
        let success : bool = execute(args);

        if !success {
            break;
        }
    }
}

fn read_line() -> String {
    let line : String = "arg1 arg2 arg3".to_string();
    line
}

fn split_args(input : String) -> Vec<String> {
    let args : Vec<String> = vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()];
    args
}

fn execute(args : Vec<String>) -> bool {
    false
}