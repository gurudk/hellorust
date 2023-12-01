fn main() {
    println!("Hello, world!");
    println!("return vec:{:?}", ret_vec_in_func());

}

fn ret_vec_in_func<'a>()->Box<Vec<i32>>{
    let mut v = Box::new( vec![]);
    for i in 1..=3 {
        println!("{:?}",i);
        v.push(i);
    }

    v
}