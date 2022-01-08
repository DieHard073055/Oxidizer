pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summerize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn summerize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summerize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summerize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct FacebookStatus {
    pub username: String,
    pub status: String,
    pub likes: i32,
    pub comments: i32,
}

impl Summary for FacebookStatus {
    fn summerize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
pub trait Summary {
    fn summerize_author(&self) -> String;
    fn summerize(&self) -> String {
        format!("(Read more from {} ...)", self.summerize_author())
    }
}

pub fn notify<T>(item1: &T, item2: &T) -> String
    where T:Summary
{
    let x = format!("breaking news! {}", item1.summerize());
    let y = format!("breaking news 2 ! {}", item2.summerize());
    format!("{}\n{}", x, y)
}

fn traits_example_1(){
    let tweet = Tweet {
        username: String::from("@jono"),
        content: String::from("sup bruv!"),
        reply: false,
        retweet: false
    };
    let article = NewsArticle {
        author: String::from("Johnno"),
        headline: String::from("The sky is falling"),
        content: String::from("The sky is not really falling.")
    };
    let facebook_status = FacebookStatus {
        username: String::from("John"),
        status: String::from("Life has been pretty tough"),
        likes: 24,
        comments: 32,
    };

    println!("Tweet summary: {}", tweet.summerize());
    println!("Article summary: {}", article.summerize());
    println!("Facebook summary: {}", facebook_status.summerize());
}
fn main() {
    traits_example_1();
}
