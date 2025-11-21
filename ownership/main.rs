fn main() {

    let s1: String = String::from("hello"); // s1 is the owner of the String
    // We pass a reference to s1 by using &
    let length: usize = calcular_length(&s1);
    println!("The length of '{}' is {}.", s1, length);

    // Move ownership example
    let s2: String = s1; // s1 is moved to s2, s1 is no longer valid
    // s1 can no longer be used here. It be cleaned up automatically.
    // println!("{}", s1); // This line would cause a compile-time error
    println!("s2 is now the owner of the String: '{}'", s2);

}

// This function takes a reference to a String and returns its length
fn calcular_length(s: &String) -> usize {
    s.len()
}

// s2 goes out of scope here and is dropped automatically
fn print_lost() {
    // println!("Lost string: {}", s2); // This line would cause a compile-time error
    //  ^^ not found in this scope
}

// Ownership in Rust is a set of rules that governs how memory is managed.
// The main concepts of ownership are:
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
