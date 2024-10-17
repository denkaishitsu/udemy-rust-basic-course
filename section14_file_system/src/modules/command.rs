use std::env;

pub fn run() {
    println!("command");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
