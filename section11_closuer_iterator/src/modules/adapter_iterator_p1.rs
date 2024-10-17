pub fn run() {
    println!("method iterator p1. Map & Filter");

    let v = vec![1, 2, 3, 4, 5];
    let m = v.iter().map(|x: &i32| x * 2);
    for i in m {
        println!("{}", i);
    }

    let c: Vec<_> = v.iter().map(|x: &i32| x * 2).collect();
    println!("{:?}", c);

    let f: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 != 0).collect();
    println!("{:?}", f);
}
