fn main() {
    println!("Hello, world!");
}

pub fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {

    #[test]
    fn first_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn second_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("Two + two does nt equal to 4".to_string())
        }
    }
}
