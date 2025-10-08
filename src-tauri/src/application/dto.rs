// Application Layer - データ転送オブジェクト

use crate::domain::entities::Book;
use async_graphql::SimpleObject;

/// Book DTO - GraphQLレスポンス用
#[derive(Debug, Clone, SimpleObject)]
pub struct BookDto {
    pub id: i32,
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub published_year: Option<i32>,
}

impl From<Book> for BookDto {
    fn from(book: Book) -> Self {
        Self {
            id: book
                .id()
                .expect("Book must have an ID when converting to DTO"),
            title: book.title().to_string(),
            author: book.author().map(String::from),
            description: book.description().map(String::from),
            published_year: book.published_year(),
        }
    }
}

