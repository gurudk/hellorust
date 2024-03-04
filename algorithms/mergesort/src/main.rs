use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("Hello, world!");
    let mut arr = [1, 2, 4, 8, 11, 5, 4, 9, 7, 12];
    println!("{:?}", arr);
    mergesort(&mut arr);

    println!("{:?}", arr);

    let mut arr: [i32; 20] = [0; 20];

    let mut rng = rand::thread_rng();
    let uni = Uniform::from(1..100);
    for i in 0..arr.len() {
        arr[i] = uni.sample(&mut rng);
    }

    println!("{:?}", arr);
    mergesort(&mut arr);
    println! {"ordered arr:{:?}",arr};
}

fn mergesort(data: &mut [i32]) {
    if data.len() <= 1 {
        return;
    }

    if data.len() == 2 {
        println!("length=2:{:?}", data);
        if data[0] > data[1] {
            data.swap(0, 1);
        }
        return;
    }

    let mid = data.len() >> 1;
    mergesort(&mut data[..mid]);
    mergesort(&mut data[mid..]);
    merge(data, mid);
}

fn merge(data: &mut [i32], mid: usize) {
    let mut i = 0;
    let mut k = mid;
    let mut end = data.len();
    let mut temp = Vec::new();

    while i < mid || k < end {
        if i < mid && (k == end || data[i] < data[k]) {
            temp.push(data[i]);
            i += 1;
        } else {
            if k < end && (i == mid || data[i] >= data[k]) {
                temp.push(data[k]);
                k += 1;
            }
        }
    }

    println!(
        "data.len:{}, temp.len:{}, data:{:?},temp:{:?}",
        data.len(),
        temp.len(),
        data,
        temp
    );
    for j in 0..data.len() {
        data[j] = temp[j];
    }
}
