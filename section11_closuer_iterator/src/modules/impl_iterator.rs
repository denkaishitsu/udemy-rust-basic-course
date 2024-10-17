struct Counter {
    start: u32,
    end: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.start > self.end {
            None
        } else {
            let result: Option<u32> = Some(self.start);
            self.start += 1;
            result
        }
    }
}

pub fn run() {
    println!("====impl Counter fn in Iterator trait.");

    let mut c: Counter = Counter { start: 1, end: 5 };
    for i in c {
        println!("{}", i);
    }

    let mut c: Counter = Counter { start: 1, end: 5 };
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next());
    println!("{:?}", c.next()); //None
    println!("{:?}", c.next()); //None
}
