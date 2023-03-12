use p22_trait::Tweet;
use p22_trait::Summary;

fn main() {
    let t = Tweet {
        username: String::from("hello rust"),
        content: String::from("hello world , hello rust!"),
        reply: String::from("666"),
        retweet: String::from("777"),
    };

    println!("{}", t.summarize());
}
