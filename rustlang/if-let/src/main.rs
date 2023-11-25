#![allow(unused)]
fn main() {
    let dish = ("Ham", "Eggs");
    let have_bacon = ("Eggs", "Bacon");

// this body will be skipped because the pattern is refuted
    if let ("Bacon", b) = have_bacon {
        println!("Bacon is served with {}", b);
    } else {
        // This block is evaluated instead.
        println!("No bacon will be served");
    }

// this body will execute
    if let ("Ham", b) = dish {
        println!("Ham is served with {}", b);
    }

    if let _ = 5 {
        println!("Irrefutable patterns are always true");
    }


    let x = Some(3);
    let a = if let Some(1) = x {
        1
    } else if x == Some(2) {
        2
    } else if let Some(y) = x {
        y
    } else {
        -1
    };
    assert_eq!(a, 3);

    enum E {
        X(u8),
        Y(u8),
        Z(u8),
    }
    let v = E::Y(12);
    if let E::X(n) | E::Y(n) = v {
        assert_eq!(n, 12);
    }
}
