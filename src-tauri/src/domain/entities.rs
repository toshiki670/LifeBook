// Domain Layer - エンティティ

use super::errors::DomainError;
use chrono::Datelike;

/// Book エンティティ（ビジネスルールを持つドメインモデル）
#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    id: Option<i32>,
    title: String,
    author: Option<String>,
    description: Option<String>,
    published_year: Option<i32>,
}

impl Book {
    /// 新しい本を作成（バリデーション付き）
    pub fn new(
        title: String,
        author: Option<String>,
        description: Option<String>,
        published_year: Option<i32>,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: タイトルは必須で、空文字列は不可
        if title.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Title cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: タイトルは200文字以内
        if title.len() > 200 {
            return Err(DomainError::ValidationError(
                "Title must be 200 characters or less".to_string(),
            ));
        }

        // ビジネスルール: 出版年は1000年から現在年まで
        if let Some(year) = published_year {
            let current_year = chrono::Utc::now().year();
            if year < 1000 || year > current_year + 1 {
                return Err(DomainError::ValidationError(format!(
                    "Published year must be between 1000 and {}",
                    current_year + 1
                )));
            }
        }

        Ok(Self {
            id: None,
            title: title.trim().to_string(),
            author: author
                .map(|a| a.trim().to_string())
                .filter(|a| !a.is_empty()),
            description: description
                .map(|d| d.trim().to_string())
                .filter(|d| !d.is_empty()),
            published_year,
        })
    }

    /// 既存の本を再構築（DB から取得した場合など）
    pub fn reconstruct(
        id: i32,
        title: String,
        author: Option<String>,
        description: Option<String>,
        published_year: Option<i32>,
    ) -> Self {
        Self {
            id: Some(id),
            title,
            author,
            description,
            published_year,
        }
    }

    /// 本の詳細を更新（バリデーション付き）
    pub fn update_details(
        &mut self,
        title: Option<String>,
        author: Option<String>,
        description: Option<String>,
        published_year: Option<i32>,
    ) -> Result<(), DomainError> {
        // タイトル更新
        if let Some(new_title) = title {
            if new_title.trim().is_empty() {
                return Err(DomainError::ValidationError(
                    "Title cannot be empty".to_string(),
                ));
            }
            if new_title.len() > 200 {
                return Err(DomainError::ValidationError(
                    "Title must be 200 characters or less".to_string(),
                ));
            }
            self.title = new_title.trim().to_string();
        }

        // 著者更新
        if let Some(new_author) = author {
            self.author = if new_author.trim().is_empty() {
                None
            } else {
                Some(new_author.trim().to_string())
            };
        }

        // 説明更新
        if let Some(new_description) = description {
            self.description = if new_description.trim().is_empty() {
                None
            } else {
                Some(new_description.trim().to_string())
            };
        }

        // 出版年更新
        if let Some(year) = published_year {
            let current_year = chrono::Utc::now().year();
            if year < 1000 || year > current_year + 1 {
                return Err(DomainError::ValidationError(format!(
                    "Published year must be between 1000 and {}",
                    current_year + 1
                )));
            }
            self.published_year = Some(year);
        }

        Ok(())
    }

    // Getters
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn published_year(&self) -> Option<i32> {
        self.published_year
    }

    // IDをセット（リポジトリで保存後に使用）
    pub(crate) fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_book() {
        let book = Book::new(
            "The Rust Programming Language".to_string(),
            Some("Steve Klabnik".to_string()),
            None,
            Some(2018),
        );
        assert!(book.is_ok());
    }

    #[test]
    fn test_empty_title_fails() {
        let book = Book::new("".to_string(), None, None, None);
        assert!(book.is_err());
    }

    #[test]
    fn test_long_title_fails() {
        let long_title = "a".repeat(201);
        let book = Book::new(long_title, None, None, None);
        assert!(book.is_err());
    }

    #[test]
    fn test_invalid_year_fails() {
        let book = Book::new("Test".to_string(), None, None, Some(500));
        assert!(book.is_err());
    }
}
