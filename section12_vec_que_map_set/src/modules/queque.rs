use std::collections::{BinaryHeap, VecDeque};

pub fn run() {
    println!("queque.");

    let mut q: VecDeque<_> = VecDeque::new();

    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    println!("{:?}", q);

    println!("{:?}", q.pop_front());
    println!("{:?}", q);

    let mut bh: BinaryHeap<_> = BinaryHeap::new();
    bh.push(1);
    bh.push(10);
    bh.push(2);
    bh.push(30);
    bh.push(5);

    println!("{:?}", bh);
    println!("{:?}", bh.pop());
    println!("{:?}", bh);
}
