fn main() {
    println!("Hello, world!");

    let user = User::new(
        String::from("as@as"),
        String::from("pass"),
        String::from("sri"),
    );

    print_contents(&user);

    let point = Point(-1, 1);

    println!("Points {} {}", point.0, point.1);

    println!("{:#?}", user);
}

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    password: String,
}

impl User {
    fn new(email: String, password: String, username: String) -> Self {
        Self {
            password,
            email,
            username,
        }
    }
}

struct Point(i32, i32);

fn print_contents(user: &User) {
    println!(
        "Username {} , Password {}, Email {}",
        user.username, user.password, user.email
    );
}
