pub fn run() {
    println!("this is iterator.");

    let v = vec![1, 2, 3, 4, 5];
    let v1_iter = v.iter();
    for x in v1_iter {
        println!("{}", x);
    }

    let vs = vec!["a", "b", "c", "d", "e"];
    let v1_iter = vs.iter();
    for x in v1_iter {
        println!("{}", x);
    }

    //共有参照
    let mut v2_iter = v.iter();
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());
    println!("{:?}", v2_iter.next());

    //可変参照
    let mut v = v;
    let mut v3_iter = v.iter_mut();
    for x in v3_iter {
        let y = &*x + 10;
        let mut z = *x;
        *x = y;
        z = z + 20;
        println!("{}, {}, {}", x, y, z);
    }
    println!("{:?}", v);
}
