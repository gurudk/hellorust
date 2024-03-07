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
    counting_sort(&mut arr);
    println! {"ordered arr:{:?}",arr};
}

fn counting_sort(data: &mut [i32]) {
    if data.len() < 2 {
        return;
    }

    let mut min = data[0];
    let mut max = data[0];

    for i in 0..data.len() {
        if data[i] < min {
            min = data[i];
        }
        if data[i] > max {
            max = data[i];
        }
    }

    let cap = (max - min + 1) as usize;
    let mut counters: Vec<usize> = vec![0; cap];

    for i in 0..data.len() {
        let counter_idx = (data[i] - min) as usize;
        if let Some(elem) = counters.get_mut(counter_idx) {
            *elem += 1;
        }
    }

    println!("counters:{:?}", counters);
    let mut ret = Vec::new();
    for i in 0..counters.len() {
        if counters[i] > 0 {
            for j in 0..counters[i] {
                ret.push(i as i32 + min);
            }
        }
    }

    data.clone_from_slice(&ret);
}
