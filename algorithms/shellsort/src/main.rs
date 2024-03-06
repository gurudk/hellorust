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
    shell_sort(&mut arr);
    println! {"ordered arr:{:?}",arr};
}

fn shell_sort(data: &mut [i32]) {
    if data.len() < 2 {
        return;
    }

    let start = 0;
    let end = data.len();
    let mut gap = data.len() >> 1;

    while gap > 0 {
        for i in 0..gap {
            //order every sequence with head i
            let mut curr = i + gap;

            println!("begin:gap={},i={},curr={},data={:?}", gap, i, curr, data);

            while curr < end {
                println!("curr<end:{},curr:{},end:{}", curr < end, curr, end);
                let mut found_pos = curr;
                let mut pos = i;

                //to find inserted position for the current element
                while pos <= curr - gap {
                    if data[curr] < data[pos] {
                        found_pos = pos;
                        break;
                    }

                    pos += gap;
                }

                //move and put right pos
                if found_pos < curr {
                    let temp = data[curr];
                    let mut temp_idx = curr;
                    //move backward
                    while temp_idx > found_pos {
                        //move
                        data[temp_idx] = data[temp_idx - gap];
                        temp_idx -= gap;
                    }
                    data[found_pos] = temp;
                }

                println!("gap:{},curr:{},data:{:?}", gap, curr, data);

                curr += gap;
                println!(
                    "add gap curr:{},gap:{}, end:{}, curr<end:{}",
                    curr,
                    gap,
                    end,
                    curr < end
                );
            }

            println!("end:gap={},i={},curr={},data={:?}", gap, i, curr, data);

            println!("======================================");
            println!("");
        }

        gap = gap >> 1;
    }
}

fn insert_last(subseq: &mut [i32]) {
    if subseq.len() < 2 {
        return;
    }

    let last = subseq.len() - 1;
    let subseq_end = last - 1;
    let mut found_pos = last;
    for i in 0..=subseq_end {
        if subseq[last] < subseq[i] {
            found_pos = i;
            break;
        }
    }

    //insert last element to found_pos
    if found_pos != last {
        let temp = subseq[last];
        for i in (found_pos..=subseq_end).rev() {
            subseq[i + 1] = subseq[i];
        }

        //insert last elment to found pos
        subseq[found_pos] = temp;
    }
}
