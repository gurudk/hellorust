fn main() {
    let data = &[1,2,3,4];
    let sum = data.iter().fold(0, |init, acc| {println!("init:{}",acc);acc+init});

    println!{"sum:{:?}", sum};
}

