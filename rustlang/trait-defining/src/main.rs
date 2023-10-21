mod aggregator;

use crate::aggregator::{Summary, Tweet};
use crate::aggregator::notify;
use crate::aggregator::notify_g;
use crate::test1::test11;

mod test1{
    pub fn test11(){

    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    notify_g(&tweet);


}
