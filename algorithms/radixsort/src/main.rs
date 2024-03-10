use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("Hello, world!");

    println!("{}", 7 >> 1);

    // for i in 0..=0 {
    //     println!("000");
    // }
    let mut arr: [usize; 10] = [0; 10];

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

fn radix_sort(data: &mut [usize]) {
    if data.len() < 2 {
        return;
    }

    let max_num = match data.iter().max() {
        Some(&x) => x,
        None => return,
    };

    let radix = data.len().next_power_of_two();

    let mut digit: usize = 1;

    println!("radix:{}", radix);
    while digit <= max_num {
        let index_of = |x| x / digit % radix;

        let mut counter = vec![0; radix];
        for &item in data.iter() {
            counter[index_of(item)] += 1;
        }

        println!("counter:{:?}", counter);


        for i in 1..radix {
            counter[i] += counter[i - 1];
        }

        println!("counter:{:?}", counter);

        //sort
        for &item in data.to_owned().iter().rev() {
            counter[index_of(item)] -= 1;
            data[counter[index_of(item)]] = item;
        }

                println!("digit:{}, sorted data:{:?}", digit, data);

        digit *= radix;
    }
}
