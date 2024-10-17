//! document of library crate

///document say_hello
pub fn say_hello(s: &str) {
    println!("Hello lib. {}", s);
}

///**document say_goodbye func**
/// ### how to
/// ```
/// fn main() {
///   section8::say_goodbye("Good bye!!");
/// }
/// ```
pub fn say_goodbye(s: &str) {
    println!("Hello lib. {}", s);
}
