extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
// 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
    let dates: Vec<&str> = re.find_iter(hay).map(|m| m.as_str()).collect();
    assert_eq!(dates, vec![
        "1865-04-14",
        "1881-07-02",
        "1901-09-06",
        "1963-11-22",
    ]);
}