fn bubble_sort(data: &mut Vec<i32>, is_asc: bool) -> &Vec<i32> {
    for i in 0..data.len() - 1 {
        for j in i+1..data.len()  {
            if is_asc {
                if data[j] < data[i] {
                    // let temp = data[j];
                    // data[j] = data[i];
                    // data[i] = temp;
                    data.swap(i, j);
                }
            }else{
                if data[j] > data[i]{
                    data.swap(i, j);
                }
            }
        }
    }

    data
}

fn main() {
    println!("Hello, world!");
    let mut data = vec![3, 2, 434, 5, 100, 33, 25];
    bubble_sort(&mut data, false);
    println!("{:?}", data);
}
