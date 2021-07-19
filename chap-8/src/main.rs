fn main() {
    println!("Hello, world!");
    crate::sola::na::test();

    let elements: Vec<i32> = Vec::new();

    let mut v = vec![1, 3, 4];
    let first_element = &v[0];

    // v.push(7); //error

    println!("{}", first_element);

    let mut data_str = "Hello wolrd".to_string();
    let slice_str = "!!!!";
    data_str.push_str(slice_str);

    println!("{}", slice_str);

    let hello = String::from("नमस्ते");

    let hello_slice = &hello[3..6];

    println!("{}", hello);
    println!("Hello slice {}", hello_slice);
}

pub mod sola;

fn initialize_vec() -> Vec<i32> {
    Vec::new()
}

fn store_different_elements() {
    let mut diff_value_vec = vec![Store::IntValue(2), Store::StringValue(String::from("Helo"))];
    diff_value_vec.push(Store::IntValue(9));
    //diff_value_vec.push(10);
}

enum Store {
    IntValue(u32),
    StringValue(String),
}
