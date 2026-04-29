mod book;
mod summary;
mod news_article;

use crate::book::Book::Book;
use crate::summary::Summary::Summary;
use crate::news_article::NewsArticle::NewsArticle;

fn main() {
    let book = Book {
        title: String::from("Ministry of Time"),
        author: String::from("Kaliane Bradley"),
        content: String::from("Time travel works, but at what cost"),
    };
    let article = NewsArticle{
        headline: String::from("Man bites dog"),
        byline: String::from("Staff (which means nobody wants credit)"),
        content: String::from("The opposite of normal"),
        edition: String::from("Evening")
    };

    let items: Vec<Box<dyn Summary>> = vec![Box::new(book), Box::new(article)];
    for item in items {
        println!("{}", item.summarize());
    }

}
