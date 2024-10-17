pub fn run() {
    println!("hello err_throw.");

    //println!("{:?}", double_even(1));

    let a: i32 = 1;

    match double_even(a) {
        Ok(val) => println!("{}", val),
        Err(err) => {
            println!("Error runにThrowされてきた");
            println!("{}", err)
        }
    };
}

fn need_even(a: i32) -> Result<i32, String> {
    if a % 2 == 0 {
        Ok(a)
    } else {
        Err(String::from("引数は偶数にしてください。"))
    }
}

fn double_even(a: i32) -> Result<i32, String> {
    // match need_even(b) {
    //     Ok(val) => Ok(val * 2),
    //     Err(err) => Err(err),
    // }
    let x = need_even(a)?; //<----?=ErrorならばResult
    Ok(x * 2)
}
