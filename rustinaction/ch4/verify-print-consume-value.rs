#[derive(Debug)]
struct SomeStruct{
    cid:u32,
}

fn main(){
    let v1:SomeStruct = SomeStruct{cid:1};
    let v2 = v1;
    println!("{:?}", v2);
    println!("{:?}", v1);
}

