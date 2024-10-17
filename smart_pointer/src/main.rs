use std::rc::Rc;

fn main() {
    println!("Hello, world! [smart pointer]");

    //Box
    println!("================:Box");
    let x: Box<i32> = Box::new(1);
    println!("x: {:p}", x);
    println!("*x + 2 = {}", *x + 2);

    //Rc
    println!("================:Rc");
    let a: Rc<String> = Rc::new("hello".to_string());
    println!("count1 &a: {}", Rc::strong_count(&a));
    {
        let b: Rc<String> = Rc::clone(&a);
        println!("a: {:p}", a);
        println!("b: {:p}", b);
        println!("count2 &a: {}", Rc::strong_count(&a));
        println!("count2 &b: {}", Rc::strong_count(&b));
    }
    println!("count3 &a: {}", Rc::strong_count(&a));
}
