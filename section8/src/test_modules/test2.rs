pub mod sub_module {
    pub fn test_fn1() {
        println!("call test2::sub_module1::test_fn1()");
    }

    //
    fn test_fn2() {
        println!("call test2::sub_module1::test_fn2()");
    }
}

mod sub_module2 {
    pub fn test_fn1() {
        println!("call test2::sub_module2::test_fn1()");
    }

    //
    pub fn test_fn2() {
        println!("call test2::sub_module2::test_fn2()");
    }
}
