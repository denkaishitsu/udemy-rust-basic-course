use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    println!("file writer.");

    let p = String::from("src/sample_writer.txt");
    let f1: File = File::create(&p);
}
