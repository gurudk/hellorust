#![allow(unused)]
fn main() {
    fn f<'a, 'b, 'c>(x: &'a i32, mut y: &'b i32) where 'a: 'b, 'c:'a {
        y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
        let r: &'b &'a &'c i32 = &&&0;   // &'b &'a i32 is well formed because 'a: 'b
    }
}