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
    bucket_sort(&mut arr, 5);
    println! {"ordered arr:{:?}",arr};
}

fn bucket_sort(data: &mut [i32], bucket_num: usize) {
    if data.len() < 2 {
        return;
    }

    let mut minV: i32 = data[0];
    let mut maxV: i32 = data[0];
    let end = data.len();
    for i in 0..end {
        if data[i] < minV {
            minV = data[i];
        }

        if data[i] < maxV {
            maxV = data[i];
        }
    }

    //
    let mut buckets: Vec<Bucket> = Vec::new();

    for i in 0..bucket_num {
        buckets.push(Bucket {
            bucket_no: i,
            values: Vec::new(),
        });
    }

    for i in 0..end {
        //search bucket and put it
        let bucket_no = i % bucket_num;
        let result = buckets.binary_search_by(|probe| probe.bucket_no.cmp(&bucket_no));
        match result {
            Ok(idx) => {
                buckets[idx].values.push(data[i].clone());
            }
            Err(idx) => {}
        }
    }

    for bucket in buckets.iter_mut(){
        bucket.values.sort();
    }

    println!("before order buckets:{:?}", buckets);

    let mut ret = Vec::new();
    loop {
        match peek_small(&buckets) {
            Some(idx) => {
                ret.push(buckets[idx].values.remove(0));
            }
            None => {
                break;
            }
        }
    }

    println!("ret:{:?}", ret);
    println!("after buckets:{:?}", buckets);
    data.clone_from_slice(&ret);
}

fn peek_small(buckets: &Vec<Bucket>) -> Option<usize> {
    let mut idx = 0;
    let mut small = i32::MAX;
    let mut is_empty = true;
    for i in 0..buckets.len() {
        if !buckets[i].values.is_empty() {
            is_empty = false;
            if buckets[i].values[0] < small {
                small = buckets[i].values[0];
                idx = i;
            }
        }
    }

    //return the smallest bucket index in pos 0;
    if !is_empty {
        Some(idx)
    } else {
        None
    }
}

#[derive(Debug)]
struct Bucket {
    bucket_no: usize,
    values: Vec<i32>,
}
