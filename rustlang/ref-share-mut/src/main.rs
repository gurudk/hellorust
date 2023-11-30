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
    // extend(&mut wave, &wave);

    println!("{:?}", wave);

    let mut  x = 20;
    let m1 = &mut x;
    *m1 = 30;
    let m2 = &mut x;
    *m2 = 100;
    println!("x={}", x);
    // let m33 = &mut m2;
    // **m33 = 333;
    // println!("x33={}",x);

    let mut v = (534,454);
    let m = &mut v;
    // let m1 = &mut v;
    let m0 = &mut m.0;
    //m.0 = 555;
    println!("v1:{:?}", m.1);
    *m0 = 666;
    println!("v:{:?}", (*m).1);
    println!("m0:{:?}", m0);

}

fn extend(vec:&mut Vec<i64>, slice:&[i64]){
    for elf in slice{
        vec.push(*elf);
    }
}
