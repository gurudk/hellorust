#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat<'a> {
    id: u64,

    name:&'a str,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main () {
    let sat_a = CubeSat { id: 0 , name:"a"};
    let sat_b = CubeSat { id: 1 , name:"b"};
    let sat_c = CubeSat { id: 2 , name:"c"};

    let sat_a = check_status(sat_a);   // <1>
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    // "waiting" ...

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
}