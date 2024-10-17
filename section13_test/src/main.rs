mod modules;

fn main() {
    println!("Hello, world! section13 test=============");

    modules::document_test::run();

    modules::unit_test::run();
    modules::panic_test::run();
}
