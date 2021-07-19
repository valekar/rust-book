use std::error::Error as std_error;
use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Box<dyn std_error>> {
    println!("Hello, world!");

    let v = vec![1, 2, 4];

    // &v[99];
    //open_file();
    open_file_with()?;
    Ok(())
}

fn open_file() {
    let f = File::open("heelo.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("heelo.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening file {:?}", other_error)
            }
        },
    };
}

fn open_file_with() -> Result<String, Error> {
    let mut f = File::open("hello.text")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
