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
    binsert_sort(&mut arr);
    println! {"ordered arr:{:?}",arr};
}

fn binsert_sort(data: &mut [i32]) {
    let end = data.len();
    if end <= 1 {
        return;
    }

    for i in 1..end {
        let curr = data[i];
        let mut left = 0;
        let mut right = i - 1;
        let mut find_index = 0;
        println!("deal with index {} element {}",i, data[i]);

        //[left ,mid][mid+1,right]
        while left <= right {
            let mut mid = (left + right) >> 1;

            println!("begin:[{},{}],mid:{}",left,right,mid);
            if left == right {
                if curr >= data[left] {
                    find_index = left + 1;
                } else {
                    find_index = left;
                }

                let mut j = i;
                println!("findindex:{},j:{}", find_index, j);
                while j > find_index {
                    data[j] = data[j - 1];
                    j -= 1;
                }
                data[find_index] = curr;

                break;
            }

            if curr <= data[mid] {
                right = mid;
                println!("left:[{left},{right}]");
            } else {
                left = mid+1;
                println!("right:[{left},{right}]");
            }
        }

        println!("======{}:{:?}",i,data);
    }
}
