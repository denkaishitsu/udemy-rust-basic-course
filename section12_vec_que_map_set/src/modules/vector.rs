pub fn run() {
    println!("=======vector. p1");

    let v1: Vec<&str> = vec!["Rust", "Golang", "NodeJS"];
    println!("{:?}", v1.as_ptr());
    println!("{:?}", v1.len());
    println!("{:?}", v1.capacity());

    let v2: Vec<&str> = vec!["Rust", "Golang", "NodeJS"];
    println!("{:?}", &v2[0]);
    //println!("{:?}", &v2[3]); //out of index Panic
    println!("{:?}", v2.get(0));
    println!("{:?}", v2.get(3)); //out of index None

    let mut v3: Vec<&str> = vec!["Rust", "Golang", "NodeJS"];
    v3.push("Python");
    println!("{:?}", v3);
    let val: Option<&str> = v3.pop();
    println!("{:?}", val);
    println!("{:?}", v3);

    let mut v4: Vec<&str> = vec!["Rust", "Golang", "NodeJS"];
    v4.insert(1, "Python");
    println!("{:?}", v4);
    v4.remove(2);
    println!("{:?}", v4);

    println!("=======vector. p2");
    let v1: Vec<&str> = vec!["Rust", "Golang", "NodeJS"];
    let v2: Vec<&str> = vec!["Python", "PHP"];
    let v3: Vec<&str> = [v1, v2].concat();
    println!("{:?}", v3);

    let (v4, v5) = v3.split_at(2);
    println!("v4 {:?}", v4);
    println!("v5 {:?}", v5);

    let mut v6: Vec<i32> = vec![3, 1, 6, 7, 2, 4, 9];
    v6.sort();
    println!("{:?}", v6);
    v6.reverse();
    println!("{:?}", v6);

    #[derive(Debug)]
    struct S {
        val1: i32,
        val2: i32,
    }
    let mut v7: Vec<S> = vec![
        S { val1: 3, val2: 1 },
        S { val1: 1, val2: 3 },
        S { val1: 2, val2: 2 },
    ];
    v7.sort_by_key(|s: &S| s.val1);
    println!("{:?}", v7);
    v7.sort_by_key(|s: &S| s.val2);
    println!("{:?}", v7);

    let v8: Vec<i32> = vec![3, 1, 6, 7, 2, 4, 9];
    println!("{:?}", v8.contains(&1));
    println!("{:?}", v8.contains(&5));

    let x: Option<usize> = v8.iter().position(|x: &i32| *x == 1);
    println!("{:?}", x);
    let x: Option<usize> = v8.iter().position(|x: &i32| *x == 5);
    println!("{:?}", x);
}
