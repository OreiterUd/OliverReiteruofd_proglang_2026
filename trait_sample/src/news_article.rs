pub mod NewsArticle {
    pub struct NewsArticle {
        pub headline: String,
        pub byline: String,
        pub content: String,
        pub edition: String,
    }
    impl crate::summary::Summary::Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!(
            "{} {}, {} {}",
            self.headline,
            self.edition,
            self.byline,
            self.content
        )
        }

    }
}
