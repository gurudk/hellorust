use std::slice;
use std::str;

fn main() {
    println!("Hello, world!");
    let story = "hello, the world!";
    let ptr = story.as_ptr();
    let len = story.len();

    assert_eq!(17, len);
    println!("{:?}:{:?}", ptr, len);

    let s = unsafe {
        // First, we build a &[u8]...
        let slice = slice::from_raw_parts(ptr, len);

        // ... and then convert that slice into a string slice
        str::from_utf8(slice)
    };
    println!("{:?}", s)
    // assert_eq!(s, Ok(story));
}
