
static mut STASH:&i32 = &128;
fn main() {
    println!("Hello, world!");
}

fn f(p: &'static i32){
    unsafe {
        STASH = p;
    }
}
