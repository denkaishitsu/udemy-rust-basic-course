use std::collections::HashMap;

pub fn run() {
    println!("MAP");

    let mut m: HashMap<&str, i32> = HashMap::new();

    m.insert("JPN", 1);
    m.insert("CHN", 2);
    m.insert("TWN", 3);
    m.insert("GER", 4);
    println!("{:?}", m);

    m.insert("GER", 10);
    println!("{:?}", m);

    println!("{:?}", m.get("JPN"));

    println!("{:?}", m.remove("CHN"));
    println!("{:?}", m);

    for (k, v) in m {
        println!("key is {:?}, val is {:?}", k, v);
    }
}
