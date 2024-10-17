pub fn run() {
    println!("hello err_is,ok...");

    println!("{}", "====is_ok(), is_err by() 1====");
    let s: Result<i32, String> = need_even(1);
    println!("{}", s.is_ok());
    println!("{}", s.is_err());
    println!("{:?}", s);

    println!("{}", "====is_ok(), is_err() by 2====");
    let s: Result<i32, String> = need_even(2);
    println!("{}", s.is_ok());
    println!("{}", s.is_err());
    println!("{:?}", s);

    println!("{}", "====err() by 1====");
    let s: Result<i32, String> = need_even(1);
    println!("{:?}", s.err());
    //println!("{:?}", s.ok());
    //println!("{:?}", s);
    println!("{}", "====ok() by 1====");
    let s: Result<i32, String> = need_even(1);
    println!("{:?}", s.ok());
    //println!("{:?}", s.err());
    //println!("{:?}", s);

    println!("{}", "====err() by 2====");
    let s: Result<i32, String> = need_even(2);
    println!("{:?}", s.err());
    //println!("{:?}", s.ok());
    //println!("{:?}", s);
    println!("{}", "====ok() by 2====");
    let s: Result<i32, String> = need_even(2);
    println!("{:?}", s.ok());
    //println!("{:?}", s.err());
    //println!("{:?}", s);
}

fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください。"))
    }
}
