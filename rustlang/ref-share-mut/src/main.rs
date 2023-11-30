fn main() {
    println!("Hello, world!");
    let v = vec![1,2,3];
    let r = &v;
    let aside = v;
    // println!("{}",r[0])


    let mut wave = Vec::new();

    let head = vec![1,2];
    extend(&mut wave, &head);
    {
        let tail = vec![3, 4];
        extend(&mut wave, &tail);
    }
    extend(&mut wave, &wave);

    println!("{:?}", wave);

}

fn extend(vec:&mut Vec<i64>, slice:&[i64]){
    for elf in slice{
        vec.push(*elf);
    }
}
