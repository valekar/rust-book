// this will not run, you can't access functions from main.rs file use lib.rs and make them binary crate
use internal_adder;

#[test]
fn test_adder() {
    assert_eq!(4, internal_adder(2, 2))
}
