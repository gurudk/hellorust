// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(mut f: F) where
    F: FnMut() {
    f();
}

fn main() {
    let mut x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = move || {x+=1;println!("{}", x);};

    apply(




    );

    println!("{}", x);
}