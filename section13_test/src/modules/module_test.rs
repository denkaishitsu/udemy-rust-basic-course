//use crate::modules::panic_test::maybe_panic;

#[cfg(test)]
mod test_modules {

    #[test]
    #[should_panic(expected = "flag is TRUE")]
    fn test_maybe_panic2() {
        crate::modules::panic_test::maybe_panic(true);
    }

    #[test]
    fn test_calc_add() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn test_calc_remove() {
        assert_eq!(1 - 1, 0);
    }
}
