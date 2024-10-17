use std::fmt::Debug;

fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd + Debug,
{
    if a >= b {
        a
    } else {
        b
    }
}

pub fn run() {
    println!("Max num ={}", max(1, 2));
}
