pub fn run() {
    println!("enum Option.");

    // enum Option<T> {
    //     None,
    //     Some<T>,
    // }

    let a: Option<i32> = Some(1);
    let b: Option<&str> = Some("str");
    let c: Option<i32> = None;

    let v: Vec<i32> = vec![1, 2, 3, 4];
    let val: Option<&i32> = v.get(3);

    println!("=======================use match");
    match val {
        Some(1) => println!("value is 1"),
        Some(x) if *x == 2 => {
            println!("value is 2")
        }
        Some(4 | 5) => println!("value is 4 or 5"),
        Some(x) => println!("value: {}", x),
        None => println!("value is None."),
    }

    println!("=======================use If let");
    if let Some(x) = val {
        println!("val={}", x);
    }
}
