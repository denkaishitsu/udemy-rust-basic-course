pub fn run() {
    println!("hello attr.");

    println!("{:?}", (1, 2, "A"));

    #[derive(Debug, PartialEq)]
    struct S {
        v1: i32,
        v2: i32,
    }
    println!("{:?}", S { v1: 1, v2: 2 });

    let s1: S = S { v1: 1, v2: 2 };
    let s2: S = S { v1: 1, v2: 2 };
    println!("{:?}", s1 == s2);
}
