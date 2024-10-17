use section9_2::sample_trait::Shape;
use section9_2::sample_trait::{double_area, Circle, Rectangle};

mod generics;

fn main() {
    println!("Hello, world! trait 2");

    let r = Rectangle {
        width: 3.0,
        height: 4.5,
    };

    let c = Circle { radius: 2.45 };

    println!("Rectangle={}", r.calc_area());
    println!("Rectangle={}", r.calc_permiter());
    Rectangle::do_something();
    println!("Rectangle={}", r.defalt_msg());
    println!("Rectangle={}", double_area(&r));

    println!("Circle={}", c.calc_area());
    println!("Circle={}", c.calc_permiter());
    Circle::do_something();
    println!("Circle={}", c.defalt_msg());
    println!("Circle={}", double_area(&c));

    generics::run();
}
