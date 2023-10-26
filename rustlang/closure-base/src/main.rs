#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

use std::thread;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //=============================================
    println!("============== closure immutably borrowing------------");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);


    //=============================================
    println!("============== closure mutably borrowing------------");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("After calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    //=============================================
    println!("============== closure taking ownership, using move keyword------------");

    // If you want to force the closure to take ownership of the values it uses
    // in the environment even though the body of the closure doesn’t strictly need ownership,
    // you can use the move keyword before the parameter list.
    //
    //     This technique is mostly useful when passing a closure to a new thread
    // to move the data so that it’s owned by the new thread. We’ll discuss threads
    // and why you would want to use them in detail in Chapter 16 when we talk about concurrency,
    // but for now, let’s briefly explore spawning a new thread using a closure
    // that needs the move keyword. Listing 13-6 shows Listing 13-4 modified to
    // print the vector in a new thread rather than in the main thread:

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn( || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // list's ownership is moved to the closure, so can't be borrowed, will be a error.
    println!("Before defining closure: {:?}", list);
}