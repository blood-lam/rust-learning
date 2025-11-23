enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Derection {
    North,
    South,
    East,
    West,
}

enum MyEnum {
    Foo,
    Bar,
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let coin = Coin::Dime;
    let cents = value_in_cents(coin);
    println!("The value of the coin is {} cents.", cents);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}.", max),
        _ => println!("The maximum is not configured."),
    }

    let direction = Derection::East;
    match direction {
        Derection::North => println!("Heading North!"),
        Derection::South => println!("Heading South!"),
        Derection::East => println!("Heading East!"),
        _ => println!("Heading West!"),
    }

    let mut count = 0;
    let v: Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for item in v {
        // if let MyEnum::Foo = item {
        // if matches!(item, MyEnum::Foo) {
        match item {
            MyEnum::Foo => count += 1,
            _ => { /* do nothing */ }
        }
    }

    println!("Counted {} Foo variants in the vector.", count);

    let p: Point = Point { x: 3, y: 10 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("First: {}, Last: {}", first, last),
        _ => todo!(),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}