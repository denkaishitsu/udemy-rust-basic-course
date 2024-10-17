use std::collections::HashSet;

pub fn run() {
    println!("===============set.");

    let mut set1: HashSet<i32> = HashSet::new();
    set1.insert(1);
    set1.insert(1);
    set1.insert(1);
    println!("set1::{:?}", set1);
    set1.insert(21);
    set1.insert(3);
    set1.insert(4);
    println!("set1::{:?}", set1);

    let dd: i32 = 21;
    println!("set1 contains::{:?}", set1.contains(&dd));
    println!("set1::{:?}", set1);
    println!("set1 remove::{:?}", set1.remove(&dd));
    println!("set1::{:?}", set1);

    let mut set2: HashSet<i32> = HashSet::new();
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);
    set2.insert(5);
    println!("set2::{:?}", set2);

    let set3 = &set1 | &set2;
    println!("set3 |::{:?}", set3);

    let set4 = &set1 & &set2;
    println!("set4 &::{:?}", set4);

    let set5 = &set1 - &set2;
    println!("set5 -::{:?}", set5);

    let set6 = &set1 ^ &set2;
    println!("set6 ^::{:?}", set6);
}
