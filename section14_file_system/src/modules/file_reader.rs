use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;

pub fn run() {
    println!("file reader.");

    //path
    let p = String::from("src/sample.txt");

    //file open & error
    //let mut f = File::open(p).unwrap();
    let f: Result<std::fs::File, std::io::Error> = File::open(&p);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("file open Error!!::{:?}", error)
        }
    };

    //file read
    let mut contents: String = String::new();
    f.read_to_string(&mut contents).unwrap();
    println!("ファイルを読み込み\n{}", contents);

    //
    //
    println!("=====1行で一気に文字列としてファイル読み込み====");
    //read & error
    let contents: String = fs::read_to_string(&p).unwrap();
    println!("ファイルを一気に読み込み\n{}", contents);

    //
    //
    println!("=====1行づつ文字列としてファイル読み込み====");

    //exp 1
    let f: Result<std::fs::File, std::io::Error> = File::open(&p);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("file open Error!!::{:?}", error)
        }
    };
    let mut buf_reader: BufReader<File> = BufReader::new(f);
    let mut line = String::new();
    buf_reader.read_line(&mut line).unwrap();
    println!("add 1row:{}", line);
    buf_reader.read_line(&mut line).unwrap();
    println!("add 2row:{}", line);

    //exp2 iter
    let f: Result<std::fs::File, std::io::Error> = File::open(&p);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("file open Error!!::{:?}", error)
        }
    };
    let mut buf_reader2: BufReader<File> = BufReader::new(f);
    let lines: Lines<BufReader<File>> = buf_reader2.lines();
    let mut c: i32 = 1;
    for l in lines {
        println!("{}::{}", c, l.unwrap());
        c = c + 1;
    }

    //
    //
    println!("=====バイト列としてファイル読み込み====");
    let f: Result<std::fs::File, std::io::Error> = File::open(&p);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("file open Error!!::{:?}", error)
        }
    };
    let mut bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut bytes).unwrap();
    println!("bytes::{:?}", bytes);
    println!("bytes to UTF8::{}", String::from_utf8(bytes).unwrap());
}
