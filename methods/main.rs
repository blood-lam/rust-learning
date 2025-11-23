struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectange {
    fn can_hold(&self, other: &Rectange) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rec1 = Rectange {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rec1.area());

    let mut light = TrafficLight {
        color: String::from("Red"),
    };
    light.show_state();
    light.change_state(String::from("Green"));
    light.show_state();
    let light1 = TrafficLight::new();
    light1.show_state();

    let light_color = TrafficLightColor::Yellow;
    println!("The traffic light color is {}.", light_color.color());
}

struct TrafficLight {
    color: String,
}

impl TrafficLight {
    fn show_state(&self) {
        println!("The traffic light is {}.", self.color);
    }

    fn change_state(&mut self, new_color: String) {
        self.color = new_color;
    }

    fn new() -> Self {
        // Self {
        TrafficLight {
            color: String::from("Red"),
        }
    }
}

enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "Red",
            Self::Yellow => "Yellow",
            TrafficLightColor::Green => "Green",
        }
    }
}
