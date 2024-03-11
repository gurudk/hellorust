use rand::distributions::{Distribution, Uniform};
use std::cmp::min;

fn main() {
    println!("Hello, world!");

    println!("{}", 7 >> 1);

    // for i in 0..=0 {
    //     println!("000");
    // }
    let mut arr: [i32; 500] = [0; 500];

    let mut rng = rand::thread_rng();
    let uni = Uniform::from(1..500);
    for i in 0..arr.len() {
        arr[i] = uni.sample(&mut rng);
    }

    // let mut arr = [
    //     20, 31, 37, 46, 68, 82, 88, 7, 16, 16, 19, 23, 80, 86, 6, 13, 20, 32, 52, 86,
    // ];

    println!("{:?}", arr);
    tim_sort(&mut arr);
    // reverse_slice(&mut arr);

    assert_eq!(insert_last(&mut [13, 72, 90]), &[13, 72, 90]);
    println! {"ordered arr:{:?}",arr};
}

#[derive(Debug)]
struct Batch {
    start: usize,
    end: usize,
    size: usize,
}

impl Batch {
    fn new(start: usize, end: usize) -> Self {
        Batch {
            start: start,
            end: end,
            size: end - start + 1,
        }
    }
}

const MIN_MERGE: usize = 64;

fn tim_sort(data: &mut [i32]) {
    if data.len() < 2 {
        return;
    }

    if data.len() == 2 {
        insert_last(data);
        return;
    }

    //处理大于2个元素的情况

    let minrun = cal_minrun(data.len());
    /**** test minrun */
    // let minrun = 16;

    let end = data.len();

    let is_asc = |x, y| x < y;
    let mut curr = 1;
    let mut batch_start = curr - 1;
    let mut batch_end = batch_start + 1;
    let mut curr_asc = is_asc(data[curr - 1], data[curr]);
    let mut batch_stack = Vec::new();

    while batch_end < end - 1 {
        batch_start = curr - 1;
        batch_end = batch_start + 1;

        //处理只有1个元素和两个元素的情况
        if (end - batch_start) < 3 {
            insert_last(&mut data[batch_start..]);
            merge_batch(
                &mut batch_stack,
                Batch::new(batch_start, end - 1),
                &mut data[..],
            );
            break;
        }

        curr_asc = is_asc(data[curr - 1], data[curr]);

        while (curr + 1 < end)
            && (is_asc(data[curr - 1], data[curr]) == is_asc(data[curr], data[curr + 1]))
        {
            curr += 1;
        }

        batch_end = curr;
        //如果是逆序，此时需要就地翻转
        if !curr_asc {
            reverse_slice(&mut data[batch_start..batch_end + 1]);
        }

        let batch_num = batch_end - batch_start + 1;
        if batch_num < minrun {
            let this_batch_end = min(end - 1, batch_start + minrun - 1);
            for j in batch_end + 1..=this_batch_end {
                //扩充当前批次
                // println!("before extend batch:{:?}", &data[batch_start..j + 1]);
                insert_last(&mut data[batch_start..j + 1]);
                // println!("after extend batch:{:?}", &data[batch_start..j + 1]);
            }
            batch_end = this_batch_end;
        }
        //处理这个批次
        merge_batch(
            &mut batch_stack,
            Batch::new(batch_start, batch_end),
            &mut data[..],
        );

        curr = batch_end + 2;
    }

    // println!("batches:{:?}", batch_stack);
}

fn merge_batch(stack: &mut Vec<Batch>, C: Batch, data: &mut [i32]) {
    println!("before merge batchs:{:?}, C:{:?}", &stack, &C);
    if stack.is_empty() {
        stack.push(C);
    } else if stack.len() == 1 {
        if C.size >= stack[0].size {
            //merge A & B
            match stack.pop() {
                Some(A) => stack.push(Batch::new(A.start, C.end)),
                None => {}
            }
        } else {
            //just push C
            stack.push(C);
        }
    } else {
        match stack.pop() {
            Some(B) => match stack.pop() {
                Some(A) => {
                    if A.size <= B.size + C.size {
                        if A.size >= C.size {
                            //merge B & C
                            stack.push(A);
                            merge_batch(stack, Batch::new(B.start, C.end), data);
                            //*************此处需要递归***************** */
                        } else {
                            //merge A & B
                            stack.push(Batch::new(A.start, B.end));
                            stack.push(C);
                        }
                    } else {
                        stack.push(A);
                        if B.size <= C.size {
                            //merge B & C
                            stack.push(Batch::new(B.start, C.end));
                        } else {
                            stack.push(B);
                            stack.push(C);
                        }
                    }
                }
                None => {}
            },
            None => {}
        }
    }

    println!("after merge batchs:{:?}", &stack);
}

fn reverse_slice(slice: &mut [i32]) {
    if slice.len() < 2 {
        return;
    }

    let last = slice.len() - 1;
    let half = slice.len() >> 1;
    for i in 0..half {
        slice.swap(i, last - i);
    }
}

fn insert_last(subseq: &mut [i32]) -> &mut [i32] {
    if subseq.len() < 2 {
        return subseq;
    }

    let last = subseq.len() - 1;
    let subseq_end = last - 1;

    if subseq[last] >= subseq[subseq_end] {
        return subseq;
    }

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

    subseq
}

fn cal_minrun(len: usize) -> usize {
    //calculate the minrun
    let mut new_len = len;
    let mut r = 0;
    while new_len > MIN_MERGE {
        r |= new_len & 1;
        new_len >>= 1;
    }

    new_len + r
}
