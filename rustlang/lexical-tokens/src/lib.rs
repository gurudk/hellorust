#[cfg(test)]
mod lexical_tests{
    #[test]
    // #[should_panic]
    fn lexical_token() {
        assert_eq!(b"foo #\"# bar", br##"foo #"# bar"##);

        assert_eq!(br"H", b"H");
        //assert_ne!('H', b"H");
        assert_eq!(b"\"foo\"",br#""foo""#);
    }
}