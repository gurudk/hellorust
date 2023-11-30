fn main() {
    println!("Hello, world!");
    let s;
    {
        let parabola = [9,4,1,0,1,4,9];
        s = smallest_ref(&parabola);
        println!("the smallest:{}", *s);
        assert_eq!(*s, 0);
    }


}

fn smallest(v: Box<[i32]>) -> i32{
    let mut s = v[0];
    for r in &v[1..]{
        if *r < s{
            s = *r;
        }
    }

    s
}


fn smallest_ref(v: &[i32]) -> &i32{
    let mut s = &v[0];
    for r in &v[1..]{
        if *r < *s{
            s = r;
        }
    }

    s
}