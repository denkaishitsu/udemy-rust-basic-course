use std::{io, num::ParseIntError};

pub fn run() {
    println!("io");

    println!("Please input strings");

    let mut input: String = String::new();
    //io::stdin().read_line(&mut input).unwrap();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("{input}");
        }
        Err(error) => println!("error: {error}"),
    }

    //let num: i32 = input.trim().parse.unwrap();
    let num = input_parse_i32(&mut input);
    println!("{:?}", num);

    match num {
        Ok(val) => println!("{}", val),
        Err(err) => {
            println!("Error:{}", err);
            println!("{}", err)
        }
    };
}

fn input_parse_i32(input: &mut String) -> Result<i32, ParseIntError> {
    let num = match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(e) => return Err(e),
    };
    Ok(num)
}
