use section9_1::sample_trait::Shape;
use section9_1::sample_trait::{double_area, Circle, Rectangle, Triangle};

// use self::section9_1::sample_trait_2::Shape2;
// use section9_1::sample_trait_2::{Circle2, Triangle2};

mod derive;
mod generics;
mod generics2;

fn main() {
    println!("Hello, world! section9_1 trait");

    let r: Rectangle = Rectangle {
        width: 1.0,
        height: 2.0,
    };

    let c = Circle { radius: 3.0 };

    let t = Triangle {
        width: 4.0,
        height: 5.0,
    };

    //
    println!("======Rectangle trait");
    println!("Rectangle area = {}", r.calc_area());
    println!("Rectangle perimeter = {}", r.calc_perimeter());
    Rectangle::do_something();

    //
    println!("======Circle trait");
    println!("Circle area = {}", c.calc_area());
    println!("Circle perimeter = {}", c.calc_perimeter());
    Circle::do_something();

    //
    println!("======Default message trait");
    println!("Rectangle default msg:: {}", r.default_msg());
    println!("Circle default msg:: {}", c.default_msg());
    println!("Triangle default msg:: {}", t.default_msg());

    //
    println!("======共通実装 trait");
    println!("Rectangle double area = {}", double_area(&r));
    println!("Circle double area = {}", double_area(&c));
    println!("Triangle double area = {}", double_area(&t));

    //
    println!("======DERIVE");
    derive::run();

    //
    println!("======Generics");
    generics::run();
    generics2::run();

    //
    println!("======LESSON");
}
