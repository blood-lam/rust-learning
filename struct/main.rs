// Define a struct named User with four fields
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// This is the main entry point of the Rust program
fn main() {
    let sign_in_count = 12;
    // Creating an instance of the User struct
    let mut user = User {
        username: String::from("lam"),
        email: String::from("lam@lam.lam"),
        sign_in_count,
        active: true,
    };
    println!("Username: {}, Email: {}", user.username, user.email);

    user.active = false;
    println!("User active status: {}", user.active);

    // Using the build_user function to create a new User instance
    let user2 = build_user(String::from("lam@example.com"), String::from("lam02"));
    println!("Username: {}, Email: {}", user2.username, user2.email);

    // Using struct update syntax to create a new User instance
    let user3 = User {
        username: String::from("lam03"),
        ..user2
    };
    println!("Username: {}, Email: {}", user3.username, user3.email);

    let user4 = set_email(user3, String::from("lam@mal.com"));
    println!("Username: {}, Email: {}", user4.username, user4.email);

    // Creating instances of the tuple structs
    let origin = Point(0, 9, -3);
    check_color(origin);
}

fn check_color(p: Point) {
    let Point(r, _, b) = p;
    assert_eq!(r, 0);
    assert_eq!(p.1, 9);
    assert_eq!(b, -3);
    println!("Point color values: r={}, g={}, b={}", r, p.1, b);
}

// Function to build a User instance
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn set_email(user: User, email: String) -> User {
    User { email, ..user }
}
