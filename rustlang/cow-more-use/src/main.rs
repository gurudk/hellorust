use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>){
    for i in 0..input.len(){
        let v = input[i];
        if v<0 {
            input.to_mut()[i] = -v;
        }
    }
}



fn main() {
    println!("Hello, world!");
    let a = [0,1,2];
    let mut input = Cow::from(&a[..]);
    abs_all(&mut input);
    assert_eq!(input, Cow::Borrowed(a.as_ref()));

    let b = [0,-1,-2];
    let mut input = Cow::from(&b[..]);
    abs_all(&mut input);
    assert_eq!(input, Cow::Owned(vec![0,1,2]) as Cow<[i32]>);


    let mut c = Cow::from(vec![0,-1,-2]);
    abs_all(&mut c);
    assert_eq!(c, Cow::Owned(vec![0,1,3]) as Cow<[i32]>);

}
