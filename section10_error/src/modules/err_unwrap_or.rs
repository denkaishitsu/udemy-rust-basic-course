pub fn run() {
    println!("hello err_unwrap_or.");

    println!("{}", "====unwrap_or() by 1====");
    let s: Result<i32, String> = need_even(1);
    println!("{:?}", s.unwrap_or(0));

    println!("{}", "====unwrap_or() by 2====");
    let s: Result<i32, String> = need_even(2);
    println!("{:?}", s.unwrap_or(0));
}

fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください。"))
    }
}
