
fn main() {
    let n1 = NewsArticle {
        headline: "thema".to_string(),
        location: "moskow".to_string(),
        author: "tom".to_string(),
        content: "my text here news".to_string(),
    };
    println!("n1: {}", n1.summarize());
    n1.print_something();
    n1.pummary();
    notify(&n1);
    let my_trait = return_trait();
    my_trait.print_something();
    my_trait.pummary();
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn print_something(&self) {
        println!("print something...");
    }
}

pub trait Pummary {
    fn pummary(&self);
    fn str_pum(&self) -> String {
        "hohohoh".to_string()
    }
}

pub struct NewsArticle {
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

impl Pummary for NewsArticle {
    fn pummary(&self) {
        println!("FFFFFFFF PUMMERY");
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

// impl NewsArticle {
//     fn print_something(&self) {
//         println!("overwrite print something");
//     }
// }

pub fn notify<T>(item: &T)
    where T: Summary + Pummary
{
    println!("wowowow! {} : {}", item.summarize(), item.str_pum());
}

fn return_trait() -> impl Summary + Pummary {
    NewsArticle {
        headline: "headline".to_string(),
        location: "location".to_string(),
        author: "author".to_string(),
        content: "content".to_string(),
    }
}