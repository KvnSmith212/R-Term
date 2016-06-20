use std::process::Command;

pub fn execute(args : Vec<String>) -> bool {
    let output = Command::new(&args[0]).args(&args[1..]).output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    let res = String::from_utf8_lossy(&output.stdout);

    println!("{}", res);
    output.status.success()
}
