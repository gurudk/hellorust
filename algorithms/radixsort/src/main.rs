use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("Hello, world!");

    println!("{}", 7 >> 1);

    // for i in 0..=0 {
    //     println!("000");
    // }
    let mut arr: [i32; 20] = [0; 20];

    let mut rng = rand::thread_rng();
    let uni = Uniform::from(1..100);
    for i in 0..arr.len() {
        arr[i] = uni.sample(&mut rng);
    }

    println!("{:?}", arr);
    radix_sort(&mut arr);
    println! {"ordered arr:{:?}",arr};

    println!("{}", 100usize.next_power_of_two());
}

fn radix_sort(data: &mut [i32]) {}
