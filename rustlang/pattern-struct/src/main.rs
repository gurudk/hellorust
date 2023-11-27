#![allow(unused)]
fn main() {
    struct Point {
        x: u32,
        y: u32,
    }
    let s = Point { x: 10, y: 20 };

    match s {
        Point { x: 110, y: some_value } => println!("10 with y = {}", some_value),
        Point { x: 110, y: 20 } => println!("order 1"),
        Point { y: 20, x: 10 } => println!("order 2"),    // order doesn't matter
        Point { x: 10, .. } => (),
        Point { .. } => println!("not matched"),
    }

    struct PointTuple(
        u32,
        u32,
    );
    let t = PointTuple(1, 2);

    match t {
        PointTuple { 0: 10, 1: 20 } => (),
        PointTuple { 1: 10, 0: 20 } => (),   // order doesn't matter
        PointTuple { 0: 10, .. } => (),
        PointTuple { .. } => (),
    }

    struct Struct {
        a: i32,
        b: char,
        c: bool,
    }
    let mut struct_value = Struct { a: 10, b: 'X', c: false };

    let local_bool = match struct_value {
        Struct { a: 10, b: 'X', c: true } => {
            println!("first");
            struct_value.c
        },
        Struct { a: 10, b: 'X', ref c } => {
            println!("ref");
            *c
        },
        Struct { a: 10, b: 'X', ref mut c } =>  struct_value.c,
        Struct { a: 10, b: 'X', c: _ } =>  struct_value.c,
        Struct { a: _, b: _, c: _ } =>  struct_value.c,
    };
    let local_bool2 = local_bool;
    print_type(&local_bool2);


}

fn print_type<T>(_:&T){
    println!("{}", std::any::type_name::<T>());
}