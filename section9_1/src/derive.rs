pub fn run() {
    println!("===============DeRive");

    // #[属性名]

    println!("{:?}", (1, 2, 3));

    #[derive(Debug, PartialEq)]
    struct S {
        val1: i32,
        val2: i32,
    }

    println!("{:?}", S { val1: 1, val2: 2 });

    let s1 = S { val1: 1, val2: 2 };
    let s2 = S { val1: 1, val2: 3 };

    println!("{}", s1 == s2);
}
