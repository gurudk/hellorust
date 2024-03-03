fn insert_sort(data: &mut [i32]) {
    let end = data.len();
    if end <= 1 {
        return;
    }

    for i in 1..end {
        if data[i] < data[i - 1] {
            let mut curr = i;
            while curr > 0 {
                if data[curr - 1] > data[curr] {
                    let mut temp = data[curr];
                    data[curr] = data[curr - 1];
                    data[curr - 1] = temp;
                }

                curr -= 1;
            }
        } 
    }
}

use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("Hello, world!");

    println!("{}", 7 >> 1);

    // for i in 0..=0 {
    //     println!("000");
    // }
    let mut arr: [i32; 10] = [0; 10];

    let mut rng = rand::thread_rng();
    let uni = Uniform::from(1..100);
    for i in 0..arr.len() {
        arr[i] = uni.sample(&mut rng);
    }

    println!("{:?}", arr);
    insert_sort(&mut arr);
    println! {"ordered arr:{:?}",arr};
}
