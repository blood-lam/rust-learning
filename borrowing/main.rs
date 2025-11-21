// Referdences and Borrowing in Rust
// References allow you to refer to some value without taking ownership of it.
// Immutable references allow you to read data without modifying it.
// Mutable references allow you to modify data, but you can only have one mutable reference to a particular piece of data at a time.
// Create references using the [&] symbol.

fn main() {
    let mut s1: i8 = 10;
    let s2: &mut i8 = &mut s1; // Immutable reference to s1

    *s2 += 5; // Error: cannot assign to `*s2` which is behind a `&` reference

    println!("The value of s1 is: {}", s1);
    // println!("The value of s2 is: {}", s2); // Dereferencing happens automatically
}