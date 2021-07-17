fn main() {
    println!("Hello, world!");

    let y = 5;
    let x = y;

    println!("{} {}", x, y);

    let mut s1 = String::from("hello world");
    let s2 = &s1;
    println!("{}", s2);

    let s3 = &mut s1;
    let s4 = &mut s1;

    //println!("{}", s3);
    println!("{}", s1);

    //println!("{} {}", s1, s2);
}
