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
    println! {"ordered arr:{:?}",arr};
}

fn heap_sort(data: &mut [i32]) {
    if data.len() < 2 {
        return;
    }

    let end = data.len();
    for i in (1..end).rev() {
        build_heap(&mut data[0..i]);
    }
}
fn build_heap(data: &mut [i32]) {
    if data.len() < 2 {
        return;
    }
    let end = data.len();
    let idx = end >> 1;
    for j in 0..=idx {
        let parent = j;
        let left = 2 * parent;
        let right = 2 * parent + 1;

        if left >= end {
            //不存在左右子节点，退出
            break;
        }

        //确定较大子结点
        let mut child = left;
        if (right < end && data[right] > data[left]) {
            child = right;
        }

        if data[child] > data[parent] {
            //交换父子节点
            data.swap(parent, child);
            //迭代更新父节点
            update_parent(data, parent);
        }
    }
    data.swap(0, end - 1);
}

fn update_parent(data: &mut [i32], child: usize) {
    let mut this_child = child;
    loop {
        let mut this_parent = this_child >> 1;
        if this_child == this_parent {
            //zero index
            break;
        }

        if data[this_child] > data[this_parent] {
            data.swap(this_child, this_parent);
            this_child = this_parent;
        } else {
            break;
        }
    }
}
