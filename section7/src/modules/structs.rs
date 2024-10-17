struct Rectagle {
    width: u32,
    length: u32,
}

impl Rectagle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn new(width: u32, length: u32) -> Self {
        Rectagle { width, length }
    }
}

pub fn run() {
    println!("run structs.");

    let mut width = 5;
    let mut length = 5;

    println!("==================[struct instance]");
    let mut rect = Rectagle { width, length };
    println!("width::{}", rect.width);
    println!("length::{}", rect.length);

    rect.width = 20;
    rect.length = 20;
    println!("width::{}", rect.width);
    println!("length::{}", rect.length);

    println!("==================[implement method in struct]");
    println!("area::{}", rect.area());

    println!("==================[struct new instance with type ref method]");
    width = 100;
    length = 100;
    let rect = Rectagle::new(width, length);
    println!("width::{}", rect.width);
    println!("length::{}", rect.length);
    println!("area::{}", rect.area());
}
