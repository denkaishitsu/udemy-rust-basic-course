pub fn run() {
    println!("Hello errors.");

    println!("{}", [11, 12, 13][0]);
    println!("{}", [11, 12, 13][1]);
    //println!("{}", [11, 12, 13][10]);

    let a: [i32; 5] = [1; 5];

    for x in a {
        println!("{}", x);
    }

    let a = [1, 2, 3];
    for x in a {
        println!("{}", x);
    }

    let v = vec![111, 222, 333];
    let sv = v.clone();
    for x in v {
        println!("{}", x);
    }
    //println!("{:?}", v);
    println!("{:?}", sv);
}
