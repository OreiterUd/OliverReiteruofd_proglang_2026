use crate::summary::Summary::Summary;

pub mod Book {
    use crate::summary::Summary::Summary;

    pub struct Book {
        pub title: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for Book {
        fn summarize(&self) -> String {
            format!("{} by {}: {}",
            self.title,
            self.author,
            self.content
            )
        }
    }
}
