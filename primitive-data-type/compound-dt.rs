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
    let number_slice: &[i16] = &[-4, 12, 0, 95];
    println!("Number Slice: {:?}", number_slice);

    let string_slice: &[&str] = &["Hello", "World"];
    println!("String Slice: {:?}", string_slice);
}
