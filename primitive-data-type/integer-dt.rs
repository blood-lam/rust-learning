// Integer Data Types in Rust
/**
  * In Rust, integer data types are used to represent whole numbers.
  * They can be signed (+ and -) or unsigned (only positive values).
  * Rust provides several integer types with different sizes and ranges.
  * Here are the commonly used integer types:
  * signed integers: i8, i16, i32, i64, i128, isize
  * unsigned integers: u8, u16, u32, u64, u128, usize
 */
fn main() {
    let a: i32 = -10; // signed 32-bit integer (-2^31 to 2^31 - 1)
    let b: u32 = 10;  // unsigned 32-bit integer (0 to 2^32 - 1)
    println!("Signed integer a: {}", a);
    println!("Unsigned integer b: {}", b);

    const MAX_I8: i8 = i8::MAX; // Maximum value for i8
    const MAX_U8: u8 = u8::MAX; // Maximum value for u8
    println!("Maximum value of i8: {}", MAX_I8);
    println!("Maximum value of u8: {}", MAX_U8);

    // let overflow_i8: i8 = exceed_i8 + 1; // This will cause a compile-time error due to overflow
    // error: literal out of range for `i8`
}
