use std::ops::RangeInclusive;

fn main() {
    println!("Hello, world! fizzbuzz");

    let nums: Vec<i32> = vec![1, 3, 5, 7, 15];
    println!("{}", nums.len());

    let mut c: usize = 0;
    while c < nums.len() {
        println!("counter:{}, number:{}", c, nums[c]);
        fizzbuzz_while(nums[c]);
        c += 1;
    }

    for n in nums.into_iter() {
        println!("for match::{}", n);
        for_match(n);
    }

    fizzbuzz2(30)
}

fn fizzbuzz_while(num: i32) {
    let mod3 = num % 3;
    let mod5 = num % 5;
    if mod3 == 0 && mod5 == 0 {
        println!("fizzbuzz. {}", num);
    } else if mod3 == 0 && mod5 != 0 {
        println!("fizz. {}", num);
    } else if mod3 != 0 && mod5 == 0 {
        println!("buzz. {}", num);
    } else {
        println!("none. {}", num);
    }
}

struct Mod {
    mod3: bool,
    mod5: bool,
}
impl Mod {
    fn modmod(n: i32, m: i32) -> bool {
        if n % m == 0 {
            true
        } else {
            false
        }
    }
}
fn modmod2(n: i32, m: i32) -> bool {
    if n % m == 0 {
        true
    } else {
        false
    }
}

fn for_match(n: i32) {
    let m_t = (modmod2(n, 3), modmod2(n, 5));
    //Mod caliculate
    let m = Mod {
        mod3: Mod::modmod(n, 3),
        mod5: Mod::modmod(n, 5),
    };

    //pattern matching 1
    match m {
        Mod {
            mod3: true,
            mod5: true,
        } => println!("fizzbuzz!!"),
        Mod {
            mod3: true,
            mod5: false,
        } => println!("fizz!!"),
        Mod {
            mod3: false,
            mod5: true,
        } => println!("buzz!!"),
        _ => println!("num::{}!!", n),
    }

    match m_t {
        (true, true) => println!("::fizzbuzz!!"),
        (true, false) => println!("::fizz!!"),
        (false, true) => println!("::buzz!!"),
        _ => println!("::num::{}!!", n),
    }
}

fn fizzbuzz2(end: i32) {
    let r: RangeInclusive<i32> = 1..=end;
    for x in r {
        match (x % 3, x % 5) {
            (0, 0) => println!("fizzbuzz2::{}::fizzbuzz", x),
            (0, _) => println!("fizzbuzz2::{}::fizz", x),
            (_, 0) => println!("fizzbuzz2::{}::buzz", x),
            _ => println!("fizzbuzz2::{}::none", x),
        }
    }
}
