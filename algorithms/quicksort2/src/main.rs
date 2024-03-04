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
    quick_sort(&mut arr, 0, high);
    println! {"ordered arr:{:?}",arr};
}

// quick_sort.rs
fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }
    let mut lm = low;
    let mut rm = high;
    while lm < rm {
        // 将右标记不断左移
        while lm < rm && nums[low] <= nums[rm] {
            rm -= 1;
        }
        // 将左标记不断右移
        while lm < rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 交换左、右标记处的值
        nums.swap(lm, rm);
    }
    // 交换分割点数据
    nums.swap(low, lm);
    if lm > 1 {
        quick_sort(nums, low, lm - 1);
    }
    quick_sort(nums, rm + 1, high);
}
