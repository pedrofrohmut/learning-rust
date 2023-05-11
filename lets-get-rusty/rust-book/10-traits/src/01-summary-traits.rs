#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

pub struct Article {
    pub author:   String,
    pub headline: String,
    pub content:  String,
}

impl Summary for Article {
    // fn summarize(&self) -> String
    // {
    //     format!("{}, by {}", self.headline, self.author)
    // }

    fn summarize_author(&self) -> String
    {
        format!("{}", self.author)
    }
}

pub struct Tweet
{
    pub username: String,
    pub content:  String,
    pub reply:    bool,
    pub retweet:  bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String
    {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String
    {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary
{
    // Without default implementation. Everyone has to define an impl for it
    // fn summarize(&self) -> String;

    // With default implementation
    // fn summarize(&self) -> String
    // {
    //     String::from("(Read more...)")
    // }

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String
    {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Returns anything that implements the Summary trait (not specifying what)
fn summarizable() -> impl Summary
{
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

pub fn notify(item: &impl Summary)
{
    println!("Breaking news! {}", item.summarize());
}

// The same of above, but without the syntatic sugar
pub fn notify1<T: Summary>(item: &T)
{
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary)
{
}

pub fn notify3<T: Summary>(item1: &T, item2: &T)
{
}

pub fn notify4(item1: &(impl Summary + Display), item2: &(impl Summary + Display))
{
}

pub fn notify5<T: Summary + Display>(item1: &T, item2: &T)
{
}

pub fn my_func1<T: Display + Clone, U: Clone + Summary>(t: &T, u: &U) -> i32
{
    666
}

// Using where to unclutter the function call
pub fn my_func2<T, U>(t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Summary
{
    666
}

pub fn my_func3<T>(arg0: &T) -> u32 where T: Display + Summary
{
    666
}

fn main()
{
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello, World!"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
    let article = Article {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling"),
        content: String::from("The sky is actually fallen"),
    };

    println!("{}", article.summarize());

    notify(&article);
    notify(&tweet);

    println!("{}", summarizable().summarize());
}
