pub fn run() {
    println!("adatpter iterator part2.");

    let v = vec![1, 2, 3, 4, 5];

    let c = v.iter().count();
    println!("iterator count ={}", c);

    let s: i32 = v.iter().sum();
    let p: i32 = v.iter().product();
    println!("iterator sum ={}", s);
    println!("iterator product ={}", p);

    let max = v.iter().max();
    let min: Option<&i32> = v.iter().min();
    println!("iterator max ={:?}", max);
    println!("iterator min ={:?}", min);

    let s2: i32 = v.iter().fold(0, |sum: i32, x: &i32| sum + x);
    println!("iterator fold ={}", s2);
}
