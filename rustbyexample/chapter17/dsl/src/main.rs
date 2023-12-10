macro_rules! calculate {
    (xxx $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        xxx 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }

    calculate! {
        xxx (1 + 2) * (3 / 4)
    }
}