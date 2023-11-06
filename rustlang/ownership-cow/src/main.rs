use std::borrow::Cow;

fn delete_spaces(src: &str) -> String{
    let mut dest = String::with_capacity(src.len());
    for c in src.chars(){
        if ' ' != c{
            dest.push(c);
        }
    }
    dest
}

fn delete_spaces2<'a>(src: &'a str) -> Cow<'a, str>{
    if src.contains(' '){
        let mut dest = String::with_capacity(src.len());
        for c in src.chars(){
            if ' ' != c{
                dest.push(c);
            }
        }
        return Cow::Owned(dest);
    }
    return Cow::Borrowed(src);
}

fn main() {
    println!("Hello, world!");
    let s = "I love you";
    let res1 = delete_spaces(s);
    let res2 = delete_spaces2(s);

    println!("{res1}, {res2}");
}
