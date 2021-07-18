fn main() {
    let address = Test::V6(String::from("::1"));

    println!("{:?}", address);
}

#[derive(Debug)]
enum Test {
    V4,
    V6(String),
}
