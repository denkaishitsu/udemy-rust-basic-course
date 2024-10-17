pub fn run() {
    println!("{}", max(1.0, 5.0));
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}
