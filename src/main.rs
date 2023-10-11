
fn greet_world(){
    println!("Hello world");
    let chn= "你好，rust";
    let chn1 = "再次看见你，rust";
    let li = [chn, chn1];
    for l in li.iter(){
        println!("{}", &l)
    }

}

fn main(){
    greet_world()
}