#![allow(unused)]
fn main() {
    fn compute(input: &u32, output: &mut u32) {
        let mut temp = *output;
        if *input > 10 {
            temp = 1;
        }
        if *input > 5 {
            temp *= 2;
        }
        *output = temp;
    }

    let mut output=0;
    compute(&12, &mut output);
    println!("{:?}", output);

    let mut data = vec![1, 2, 3];
// This mut allows us to change where the reference points to
    let mut x = &data[0];

    println!("{}", x); // Last use of this borrow
    data.push(4);
    println!("{}", x); // Last use of this borrow
    x = &data[3]; // We start a new borrow here
    println!("{}", x);
}