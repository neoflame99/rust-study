

extern crate trait_exam;
use trait_exam::Tweet;
use trait_exam::NewsArticle;
use trait_exam::Summary;

fn main() {
    let tweet = Tweet {
        username : String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let tweet2 = Tweet {
        username : String::from("pearson_ebooks"),
        content: String::from("No matter"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}",tweet.summarize());

    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL"),
    };
    println!("News article available! {}", article.summarize());

 //   trait_exam::notify(tweet); // using trait by argument
    trait_exam::notify(tweet, tweet2); // using trait by argument

    //-- trait bounds
    let number_list = vec![34, 50 ,25 , 100, 65];
    let result =  largest(&number_list);
    println!("largest number is {}", result);
}

/*
//-- trait bounds ways
fn largest<T: PartialOrd + Copy >(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest {
            largest = item;
        }
    }
    largest
}*/
//-- return reference ways
fn largest<T: PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    &largest
}