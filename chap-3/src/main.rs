// fn main() {
//     //let y = x = 8;

//     println!("Hello, world!");
//     println!("{}", something_test());

//     fn something_test() -> u8 {
//         1
//     }
// }

// fn main() {
//     let x = plus_one(5);

//     println!("{}", x);
//     print!("{}", x.clone());
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}
