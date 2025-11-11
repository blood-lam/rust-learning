// Floating-Point Data Types in Rust
/**
  * In Rust, floating-point data types are used to represent numbers with decimal points.
  * Rust provides two primary floating-point types:
  * f32: 32-bit floating-point number (single precision)
  * f64: 64-bit floating-point number (double precision, default type)
  * Floating-point numbers can represent a wide range of values, including very small and very large numbers.
  * They also support special values like NaN (Not a Number) and infinity.
 */
fn main() {
    let x: f32 = 3.14; // 32-bit floating-point number
    let y: f64 = 2.718281828459045; // 64-bit floating-point number
    println!("32-bit float x: {}", x);
    println!("64-bit float y: {}", y);
}
