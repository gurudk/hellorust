use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("Hello, world!");

    let mut arr: [i32; 10] = [0; 10];

    let mut rng = rand::thread_rng();
    let uni = Uniform::from(1..100);
    for i in 0..arr.len() {
        arr[i] = uni.sample(&mut rng);
    }

    println!("{:?}", arr);
    // binsert_sort(&mut arr);
    let high = arr.len() - 1;
    // let split = partition(&mut arr, 0, high);
    quicksort(&mut arr, 0, high);
    println! {"ordered arr:{:?}",arr};
}

fn quicksort(data: &mut [i32], low: usize, high: usize) {
    if low < high {
        let split = partition(data, low, high);
        println! {"{},{},partition:{:?},split:{}",low, high, data, split};

        if split > 1 {
            quicksort(data, low, split - 1);
        }

        quicksort(data, split + 1, high);
    }
}

fn partition(data: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low;
    let mut rm = high;
    loop {
        while lm <= rm && data[lm] <= data[low] {
            lm += 1;
        }
        while lm <= rm && data[rm] >= data[low] {
            rm -= 1;
        }

        if rm < lm {
            break;
        } else {
            data.swap(lm, rm);
        }
    }

    data.swap(rm, low);
    rm
}
