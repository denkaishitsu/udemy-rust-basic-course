pub fn run() {
    println!("Hello generics 2.");

    println!("max is {}", max(1, 2));

    println!("max is {}", max(1.0, 2.0));
    println!("max is {}", max("x", "a"));
}

fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x
    } else {
        y
    }
}
