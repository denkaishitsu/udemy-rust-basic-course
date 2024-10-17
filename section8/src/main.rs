mod rand1;
mod test_modules;

use test_modules::{test1, test2, test3};
fn main() {
    println!("Hello, world! ::section8");

    rand1::run();

    println!("----------------run from test_modules.rs");
    test_modules::run();

    println!("----------------run from main.rs");
    test1::sub_module::test_fn1();
    test2::sub_module::test_fn1();
    test3::test_fn1();

    println!("----------------Public struct filed");
    let s = test3::PubTestStruct { val_1: 1, val_2: 2 };
    println!("{:?}", s);

    println!("----------------Private struct filed");
    let s = test3::PrvTestStructFields::new(3, 4);
    println!("{:?}", s);

    let l = test3::LoadCapselStructive::new(5, 6);
    println!("{:?}", l);

    println!("This is section8/main.");

    section8::say_hello("section8");
}
