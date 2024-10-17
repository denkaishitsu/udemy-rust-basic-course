use section9::sample_trait::{double_area, Circle, Rectangle, Shape};

mod attr;

fn main() {
    println!("Hello, world! section9.");

    let rec: Rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    let cir: Circle = Circle { radius: 5.0 };

    //trait doing
    println!("Rectagle Area={}", rec.calc_area());
    println!("Rectagle perimeter={}", rec.calc_perimeter());
    Rectangle::do_something();

    println!("Circle Area={}", cir.calc_area());
    println!("Circle perimeter={}", cir.calc_perimeter());
    Circle::do_something();

    //default test
    println!("Rectangle default::{}", rec.default_something());
    println!("Circle default::{}", cir.default_something());

    //using tarit to args
    println!("Rectangle Double Area={}", double_area(&rec));
    println!("Circle Double Area={}", double_area(&cir));

    attr::run();
}
