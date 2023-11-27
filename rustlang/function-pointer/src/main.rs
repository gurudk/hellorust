#![allow(unused)]
fn main() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let mut x = add(5,7);

    type Binop = fn(i32, i32) -> i32;
    let bo: Binop = add;
    x = bo(5,7);

    let want_i32 = false;
    fn foo<T>() { }

// `foo_ptr_1` has function pointer type `fn()` here
    let foo_ptr_1: fn() = foo::<i32>;

// ... and so does `foo_ptr_2` - this type-checks.
    let foo_ptr_2 = if want_i32 {
        foo::<i32>
    } else {
        foo::<u32>
    };

    fn foo_never() -> ! {
        panic!("This call never returns.");
    }

    foo_never();

}