pub fn run() {
    println!("hello err_exprct.");

    println!("{}", "====exprct() by 2====");
    let s: Result<i32, String> = need_even(2);
    println!("{:?}", s.expect("this is expect()..."));

    println!("{}", "====expect() by 1====");
    let s: Result<i32, String> = need_even(1);
    println!("{:?}", s.expect("this is expect()..."));
}

fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください。"))
    }
}
