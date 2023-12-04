struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(rb:bool) -> Box<dyn Animal> {
    if rb {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

use rand::prelude::*;
fn main() {
    let random_number = 0.234;
    let animal = random_animal(random());
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
