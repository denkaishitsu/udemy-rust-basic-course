pub fn run() {
    println!("hello err_unwrap.");

    println!("{}", "====unwrap() by 2====");
    let s: Result<i32, String> = need_even(2);
    println!("{:?}", s.unwrap());

    println!("{}", "====unwrap() by 1====");
    let s: Result<i32, String> = need_even(1);
    println!("{:?}", s.unwrap());
}

fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください。"))
    }
}
