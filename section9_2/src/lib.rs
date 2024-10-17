pub mod sample_trait {
    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_permiter(&self) -> f64;
        fn do_something();
        fn defalt_msg(&self) -> &str {
            "This is default message."
        }
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }
        fn calc_permiter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }
        fn do_something() {
            println!("This is Rectangle.");
        }
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }
        fn calc_permiter(&self) -> f64 {
            self.radius * 2.0 + self.radius * 2.0 + std::f64::consts::PI * 2.0
        }
        fn do_something() {
            println!("This is Circle.");
        }
        fn defalt_msg(&self) -> &str {
            "This is Circle default message."
        }
    }

    pub fn double_area(shape: &impl Shape) -> f64 {
        shape.calc_area() * 2.0
    }
}
