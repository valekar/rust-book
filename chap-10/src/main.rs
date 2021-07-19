fn main() {
    println!("Hello, world!");
    let list_i32 = vec![1, 5, 7, 0];

    let largest = find_largest(&list_i32);

    println!(" largest {}", largest);

    //lifetimes
    let string1 = "small";
    let mut result = "";
    println!("result initial {}", result);
    {
        let string2 = "this is thhhhe  largest onnee";

        result = longest_str(string1, string2);

        println!("The largest is {}", result);
    }
    println!("{} ", result);

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest_str(string1.as_str(), string2.as_str());  // this will error out cause string2 lifetime is shorter to be referenced out of the scope in the print statement below
    // }
    // println!("The longest string is {}", result);
}

//#[derive(PartialOrd)]
fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn find_largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn longest_str<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}
