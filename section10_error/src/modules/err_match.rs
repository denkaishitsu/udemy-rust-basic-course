pub fn run() {
    println!("hello err_match.");

    let x: i32 = match need_even(2) {
        Ok(val) => val,
        Err(err) => {
            //println!("error message: {}", err);
            panic!("error message: {}", err);
        }
    };
    println!("{}", x);
}

fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください。"))
    }
}
