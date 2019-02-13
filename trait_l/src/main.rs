
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// pub fn notify<T: impl Summary>(item:T, item2: T) {
//     item.summarize();
//     item2.summarize();
// }

pub fn notify(item1: impl Summary, item2: impl Summary) {
    println!("{}",item1.summarize());
    println!("{}", item2.summarize());
}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let news = NewsArticle {
        headline: String::from("知否，知否 应是绿肥红瘦"),
        location: String::from("Anyang Henan"),
        author: String::from("yunfeng zhang"),
        content: String::from("李清照写的"),
    };
    notify(tweet, news);
}
