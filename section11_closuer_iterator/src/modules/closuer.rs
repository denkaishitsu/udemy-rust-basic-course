pub fn run() {
    println!("this is closuer.");

    let c1 = |x: i32| x + 1;
    println!("{}", c1(10));

    let m = 10;
    let c2 = |x: i32| x + m; //m is called free variable.
    println!("{}", c2(10));

    let m = 200;
    println!("{}", c2(10)); //not yet change

    let c2 = |x: i32| x + m;
    println!("{}", c2(100)); //after def closuer, cause change m

    let v = vec![1, 2, 3];
    let c3 = || println!("{:?}", v); //this is reff.
    c3();
    println!("{:?}", v);

    //this is Error. v is value borrowed here after move
    let c3 = move || println!("{:?}", v); //move.
    c3();
    //println!("{:?}", v); //Error
}
