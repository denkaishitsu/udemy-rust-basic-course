pub fn run() {
    maybe_panic(true);
}

pub fn maybe_panic(flag: bool) {
    if flag == false {
        println!("not panic.");
        panic!("no no not");
    } else {
        panic!("====flag is TRUE!!!");
    }
}

#[test]
#[ignore]
#[should_panic(expected = "flag is TRUE")]
fn test_maybe_panic() {
    maybe_panic(true);
}

mod test_module {
    #[test]
    #[should_panic(expected = "flag is TRUE")]
    fn test_maybe_panic3() {
        super::maybe_panic(true);
    }
}
