
fn main() {
    let num: i8 = 5;
    if num > 0 {
        println!("The number is positive.");
    } else if num < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }

    let array: [i8; 5] = [10, 20, 30, 40, 50];
    for (index, value) in array.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }

    let mut count: i32 = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }

    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        println!("Odd number: {}", i);
    }

    count = 0;

    loop {

        count += 1;

        if count == 3 {
            println!("Skipping count 3");
            continue; // Skip even counts
        }
        
        if count >= 10 {
            break; // Exit the loop when count reaches 10
        }
        println!("Loop Count: {}", count);
    }

    count = 0;

    let result = loop {
        count += 1;

        if count == 5 {
            break count * 2; // Return count * 2 when breaking
        }
    };
    println!("Result from loop: {}", result);
}