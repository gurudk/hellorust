#![allow(unused)]
fn main() {
// A heap-allocated array, coerced to a slice
    let boxed_array: Box<[i32]> = Box::new([1, 2, 3]);
    let array:[i32;3] = [1,2,3];

// A (shared) slice into an array
    let slice: &[i32] = &boxed_array[..];
    let slice1:&[i32] = &array;
    print_type(&slice);
    print_type(&boxed_array);
    print_type(&array);
    print_type(&slice1);
    println!("boxed_array:{:?}", boxed_array);
    println!("slice:{:?}", slice);

    for el in slice1{
        println!("{}",el);
    }
}

fn print_type<T>(_: &T){
    println!("{}", std::any::type_name::<T>())
}