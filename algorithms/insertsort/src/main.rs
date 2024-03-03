fn insert_sort(data: &mut [i32]) {
    let end = data.len();
    if end <= 1 {
        return;
    }

    for i in 1..end {
        let temp = data[i];

        if data[i - 1] > temp {
            let mut curr = i - 1;
            loop {
                if data[curr] > temp {
                    //右移
                    data[i] = data[i - 1];
                    curr -= 1;
                } 
            }

            //当前值比小于等于排序值，把temp放在curr+1位置,排序结束
            data[curr + 1] = temp;
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
