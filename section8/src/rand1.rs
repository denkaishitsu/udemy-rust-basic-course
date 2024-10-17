use rand::Rng;

pub fn run() {
    println!("hello rand!!");

    let random_1: i32 = rand::thread_rng().gen_range(1..100);
    println!("{}", random_1);
}
