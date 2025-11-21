/** Rust Compound Data Types
 * 
 * Compound types can group multiple values into one type. Rust has two
 * primary compound types:
 * 
 * 1. Tuples: Fixed-length collection of values of different types
 * 2. Arrays: Fixed-length collection of values of the same type
 * 
 * Both tuples and arrays are stored on the stack and have a fixed size
 * at compile time.
 */

fn main() {
    // Array
    let numbers: [i16; 3] = [5, 3, -19];
    println!("Number Array: {:?}", numbers);

    // let mix = [1, 0, "No", false];
    // println!("Mix Array: {:?}", mix);
    // ^^^^ expected integer, found `&str`

    let fruits: [&str; 3] = ["apple", "banana", "kiwi"];
    println!("Fruit Array: {:?}", fruits);
    println!("Fruit Array 1st element: {}", fruits[0]);
    println!("Fruit Array 2nd element: {}", fruits[1]);
    println!("Fruit Array 3rd element: {}", fruits[2]);

    // Tuple
    // let human: (&str, i8, bool) = ("Lam", 30, false);
    let human: (String, i8, bool) = ("Lam".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let mix_tuple = ("Sister", -43, true, [5, 1, 9, 0]);
    println!("Mix Tuple: {:?}", mix_tuple);

    // Slices
    // &[T] is a slice type, representing a view into a contiguous sequence
    let number_slice: &[i16] = &[-4, 12, 0, 95];
    println!("Number Slice: {:?}", number_slice);

    // &[&str] is a slice of string slices
    let string_slice: &[&str] = &["Hello", "World"];
    println!("String Slice: {:?}", string_slice);

    let book_slice: &[&String] = &[&String::from("The Rust Programming Language"), &String::from("Programming in Rust")];
    println!("Book Slice: {:?}", book_slice);

    // String vs String slice (&str)
    // String is a heap-allocated, growable / dynamic, mutable, owned string type
    // It's wide, big, slow
    let owned_string: String = String::from("Hello, Rust!");
    println!("Owned String: {}", owned_string);
    
    // Variable immutable by default, so we need to make it mutable to modify
    let mut owned_string: String = owned_string;
    owned_string.push_str(" Welcome to compound data types.");
    println!("Modified Owned String: {}", owned_string);

    // &str is a string slice, reference to a string data, store in stack
    // stack cannot grow or shrink, so &str is fixed-length, immutable, borrowed string type
    // It's narrow, small, fast
    let heap_string: String = String::from("Hello, World!");
    let string_slice: &str = &heap_string;
    println!("String Slice: {}", string_slice);
    let index_string: &str = &heap_string[1..4]; // slice from index 1 to 3
    println!("Index String: {}", index_string);

}
