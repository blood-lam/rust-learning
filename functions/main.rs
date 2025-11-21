// main fn is entry point of the program
// functions and variable should be written in snake_case
fn main() {
    println!("Starting here!");
    let result = add(5, 3);
    println!("The sum is: {}", result);
    let example = _example();
    println!("The example value is: {}", example);
    println!("Value from add function: {}", add(10, 20));

    let weight = 54.0; // in kilograms
    let height = 1.71; // in meters
    let bmi = calculate_bmi(weight, height);
    println!("The BMI is: {:.2}", bmi);
}

// add function takes two integers and returns their sum
// Can call functions anywhere after they are defined
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Note: In Rust, the last expression in a function is the return value
// and does not require a semicolon. Adding a semicolon would turn it into
// a statement, resulting in a return type of () (unit type).

// Expressions vs Statements:
// - Statements perform actions and do not return values.
// - Expressions evaluate to a value and can be part of larger expressions.

// Example of an expression:
// let x = 5 + 3; // '5 + 3' is an expression that evaluates to 8

// Example of a statement:
// let y = 10; // This is a statement that declares a variable  'y' and assigns it the value 10
// Cannot assign the result of a statement to a variable
// let z = let a = 5; // This is invalid because 'let a = 5;' is a statement
// 1 Variable declaration is a statement: let x = 5;
// 2 Function definitions are statements: fn add(a: i32, b: i32) -> i32 { a + b; }
// Control flow constructs like if, loop, and match are also statements: if x > 5 { ... }
fn _example() -> i8 {
    let x = 10; // statement
    let y = {
        let z = 5; // statement
        z + 2      // expression, returns 7
    }; // 'y' will be assigned the value 7
    y + x // expression, returns 17
}

// Calculate BMI (Body Mass Index)
fn calculate_bmi(weight_kg: f32, height_m: f32) -> f32 {
    weight_kg / (height_m * height_m)
}
