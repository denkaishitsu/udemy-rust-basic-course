pub mod test1;
pub mod test2;
pub mod test3;
pub fn run() {
    println!("hello test modules.");

    test1::sub_module::test_fn1();
    test2::sub_module::test_fn1();
    test3::test_fn1();
}
