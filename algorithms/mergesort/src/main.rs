fn main() {
    println!("Hello, world!");
    let mut arr = [1,2,4,8,11,5,4,9,7,12];

    println!("{:?}",&arr[0..0]);

}

fn mergesort(data:&mut [i32]){
    if data.len()<=1 {
        return;
    }

    if data.len() == 2 {
        if data[0] > data[1]{
            data.swap(0,1);
        }
    }

    let mid = data.len() >> 1;
    mergesort(&mut data[..mid]);
    mergesort(&mut data[mid..]);
    merge(data, mid);

}

fn merge(data:&mut [i32], mid:usize){
    let mut i = 0;
    let mut k = mid;
    let mut end = data.len() - 1;
    let mut temp = Vec::new();

    while i<mid || k < end{
        if i<mid && (k == end || data[i] < data[k]){
            temp.push(data[i]);
            i += 1;
        }else{
            if k < end && (i == mid || data[i] >= data[k]) {
                temp.push(data[k]);
                k += 1;
            }
        }
    }

    for j in 0..data.len(){
        data[j] = temp[j];
    }
}
