/**
 * Common derived traits in Rust include:
 * - Debug: Enables formatting using the {:?} formatter.
 * - Clone: Allows for creating a copy of a value.
 * - PartialEq: Enables comparison of values for equality.
 * - Eq: A marker trait indicating that equality is reflexive.
 * - PartialOrd: Enables comparison of values for ordering.
 * - Ord: A marker trait indicating a total ordering.
 * - Hash: Allows for hashing of values, useful in hash maps and sets.
 */
trait Animal {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;

fn main() {

    let dog = Dog;
    let cat = Cat;

    animal_sound(&dog);
    animal_sound(&cat);

    let my_pet = get_animal("dog");
    println!("My pet says: {}", my_pet.speak());

    print_animal_sounds(&dog, &cat);

}

// Implementing the Animal trait for Dog
impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}

// Implementing the Animal trait for Cat
impl Animal for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
}

// trait as parameter function
fn animal_sound<T: Animal>(animal: &T) {
    println!("{}", animal.speak());
}

// Return type as trait
fn get_animal(animal_type: &str) -> Box<dyn Animal> {
    match animal_type {
        "dog" => Box::new(Dog),
        "cat" => Box::new(Cat),
        _ => panic!("Unknown animal type"),
    }
}

// Where clause example
fn print_animal_sounds<T, U>(animal1: &T, animal2: &U)
where
    T: Animal,
    U: Animal,
{
    println!("Animal 1 says: {}", animal1.speak());
    println!("Animal 2 says: {}", animal2.speak());
}
