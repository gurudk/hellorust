
fn quick_sort(data:&mut[i32], low:usize, high:usize){
    if low < high{
        let split = partition(data, low, high);
        if split > 1{
            quick_sort(data, 0, split-1);
        }

        quick_sort(data, split+1, high);
    }
}

fn partition(data:&mut[i32], low:usize, high:usize)->usize{
    let mut lm = low;let mut rm = high;
    loop{
        while lm<=rm && data[lm]<=data[low]{
            lm += 1;
        }

        while lm<=rm && data[rm]>=data[low]{
            rm -= 1;
        }
        
        if lm>rm{
            break;
        } else{
            data.swap(lm, rm)
        }
    }
    data.swap(low, rm);
    rm
}

fn main() {
    println!("Hello, world!");

    let mut list = [4,2,65,6,6,478,857,3,3,758,9523,2];
    let high = list.len() - 1;
    let pivot = quick_sort(&mut list, 0, high);
    println!("{:?}",list);

}
