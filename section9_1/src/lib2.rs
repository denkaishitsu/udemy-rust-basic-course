pub mod sample_trait_2 {
    pub trait Shape2 {
        fn get_area(&self) -> f64;
        // fn get_permiter(&self) -> f64;
        // fn do_something();
        // fn do_default(&self) -> &str {
        //     "This is default msg."
        // }
    }

    pub struct Triangle2 {
        pub width: f64,
        pub height: f64,
    }

    pub struct Circle2 {
        pub radius: f64,
    }

    impl Shape2 for Triangle2 {
        fn get_area(&self) -> f64 {
            self.width * self.height * 1 / 2
        }
    }
}
