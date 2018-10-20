

extern crate trait_exam;
use trait_exam::Tweet;
use trait_exam::Summary;

fn main() {
    let tweet = Tweet {
        username : String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}",tweet.summarize());
}
