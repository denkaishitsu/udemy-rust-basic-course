use std::fmt::format;

fn main() {
    println!("Hello, world! Owners");

    println!("==================[v1]");
    let mut v1: Vec<i32> = vec![1, 2, 3];
    println!("v1 ptr:: {:?}", v1.as_ptr());
    println!("v1[0]:: {:p}", &v1[0]);
    println!("v1 len:: {}", v1.len());
    println!("v1 capacity:: {}", v1.capacity());

    println!("==================[v1.push]");
    v1.push(4);
    println!("v1 ptr:: {:?}", v1.as_ptr());
    println!("v1[0]:: {:p}", &v1[0]);
    println!("v1 len:: {}", v1.len());
    println!("v1 capacity:: {}", v1.capacity());

    println!("==================[move to v2]");
    let v2 = v1;
    println!("v2 ptr:: {:?}", v2.as_ptr());
    println!("v2[0]:: {:p}", &v2[0]);
    println!("v2 len:: {}", v2.len());
    println!("v2 capacity:: {}", v2.capacity());

    println!("==================[clone v3]");
    let v3 = v2.clone();
    println!("v2 ptr:: {:?}", v2.as_ptr());
    println!("v2[0]:: {:p}", &v2[0]);
    println!("v2 len:: {}", v2.len());
    println!("v2 capacity:: {}", v2.capacity());
    println!("v3 ptr:: {:?}", v3.as_ptr());
    println!("v3[0]:: {:p}", &v3[0]);
    println!("v3 len:: {}", v3.len());
    println!("v3 capacity:: {}", v3.capacity());

    println!("==================[move to func]");
    let s1: String = String::from("Hello");
    let s2: String = String::from("Rust");
    let s = concat_string(s1, s2);
    println!("{}", s);
    // println!("s1::{}", s1);
    // println!("s2::{}", s2);

    println!("==================[move to func with tupple]");
    let s1: String = String::from("Hello");
    let s2: String = String::from("Rust");
    let (s, s1, s2) = concat_string2(s1, s2);
    println!("{}, {}, {}", s, s1, s2);

    println!("==================[borrow to func]");
    let mut s1: String = String::from("Hello");
    let s2: String = String::from("Rust");
    s1 = s1 + " GoGo";

    let s = concat_string3(&s1, &s2);
    println!("{}", s);
    println!("s1::{}", s1);
    println!("s2::{}", s2);

    let s = concat_string4(&s1, &s2);
    println!("{}", s);
    println!("s1::{}", s1);
    println!("s2::{}", s2);

    let s = concat_string5(&s1, &s2);
    println!("{}", s);
    println!("s1::{}", s1);
    println!("s2::{}", s2);
}

fn concat_string(a: String, b: String) -> String {
    format!("{}, {}", a, b)
}
fn concat_string2(a: String, b: String) -> (String, String, String) {
    let c: String = format!("{}+{}", a, b);
    (c, a, b)
}

fn concat_string3(a: &String, b: &String) -> String {
    format!("3::{}++{}", a, b)
}

fn concat_string4(a: &String, b: &String) -> String {
    format!("4::{}++{}", a, b)
}

fn concat_string5<'a, 'b>(a: &'a String, b: &'b String) -> String {
    format!("5::{}++{}", a, b)
}
