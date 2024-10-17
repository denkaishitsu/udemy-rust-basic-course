pub fn run() {
    println!("unit test========.");

    let a = 2;
    let b = 2;

    match eq_checker(a, b) {
        true => println!("TRUE!!"),
        false => println!("FALSE!!"),
    };
}

pub fn eq_checker(a: i32, b: i32) -> bool {
    if a == b {
        true
    } else {
        false
    }
}

#[test]
fn test_eq_check_true() {
    let a = 3;
    let b = 3;
    let r = eq_checker(a, b);

    assert!(r == true);
}
#[test]
fn test_eq_check_false() {
    let a = 2;
    let b = 3;
    let r = eq_checker(a, b);

    assert!(r == false);
}
