
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Example of an enum with explicit discriminant values
// Not accept decimal (x.x) or negative numbers (-5)
enum Number {
    Zero = 5, 
    One, // implicitly 6
    Two = 10,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Option enum is defined in the standard library
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    match home {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 Address: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6 Address: {}", addr);
        }
    }

    match loopback {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 Address: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6 Address: {}", addr);
        }
    }

    let mes1 = Message::Move { x: 10, y: 20 };
    println!("Created a Message::Move variant - {:?}", mes1);
    // match mes1 {
    //     Message::Quit => println!("Quit message"),  
    //     Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    //     Message::Write(text) => println!("Write message: {}", text),
    //     Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    // }

    if let Message::Move{x: a, y: b} = mes1 {
        println!("Destructured Move: x = {}, y = {}", a, b);
    } else {
        println!("Not a Move message");
    }

    let msgs: [Message; 4] = [
        Message::Quit,
        Message::Move { x: 5, y: 10 },
        Message::Write(String::from("Hello, world!")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);  
    println!("six: {:?}, none: {:?}", six, none);

}

fn show_message(msg: Message) {
    println!("Message received: {:?}", msg);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
