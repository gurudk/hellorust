// fn some_fn() {
//     ()
// }
//
// fn main() {
//     let _a: () = some_fn();
//     println!("This function returns and you can see this line.");
// }


#![feature(never_type)]

fn main() {
    let x: ! = panic!("This call never returns.");
    println!("You will never see this line!");
}