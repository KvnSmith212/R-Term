use std::process::Command;

pub fn execute(args : Vec<String>) -> bool {
    let output = Command::new(&args[0]).output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    output.status.success()
}
