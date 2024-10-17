#[derive(Debug)]
enum Shape {
    Circle,
    Square(u32),
    Triangle { base: u32, height: u32 },
}
impl Shape {
    fn sample_call(&self, n: &str) {
        println!("sample calling {}", n);
    }
}

pub fn run() {
    println!("hello enum.");
    println!("==================[Shape enums instance]");
    let c = Shape::Circle;
    let s = Shape::Square(12);
    let t = Shape::Triangle { base: 5, height: 7 };
    println!("{:?}\n,{:?}\n,{:?}", c, s, t);

    println!("==================[Call Shape enums method]");
    c.sample_call("Circle");
    s.sample_call("Square");
    t.sample_call("Triangle");
}
