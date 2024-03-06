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
    select_sort(&mut arr);
    println! {"ordered arr:{:?}",arr};
}

fn select_sort(data: &mut [i32]) {
    if data.len() < 2 {
        return;
    }

    let end = data.len();
    let mut sub_end = end - 1;

    while sub_end > 0 {
        let mut pos_max = 0;

        //select max index
        for i in 0..=sub_end {
            if data[i] > data[pos_max] {
                pos_max = i;
            }
        }

        data.swap(sub_end, pos_max);

        println!("sub_end:{},data:{:?}", sub_end, data);

        sub_end -= 1;
    }
}
