mod io_helper;
mod executor;

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

        let input : String = io_helper::handle_read();
        let args : Vec<String> = io_helper::split_line(&input);
        let success : bool = executor::execute(args);

        if !success {
            break;
        }
    }
}
