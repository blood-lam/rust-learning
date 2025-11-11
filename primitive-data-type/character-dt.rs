/** 
  * In Rust, character data types are used to represent single Unicode characters.
  * The primary character type in Rust is `char`, which is a 4-byte Unicode scalar value.
  * Characters are enclosed in single quotes, e.g., 'a', 'α', '中'.
  * Rust also provides a way to represent string slices with the `&str` type.
  */
fn main() {
    let c: char = 'a';
    let s: &str = "Hello, world!";
    println!("Character c: {}", c);
    println!("String slice s: {}", s);
}
