// Asset Domain - Manufacturer Entity

use crate::domain::errors::DomainError;

/// Manufacturer エンティティ（ビジネスルールを持つドメインモデル）
#[derive(Debug, Clone, PartialEq)]
pub struct Manufacturer {
    id: Option<i32>,
    name: String,
    website: Option<String>,
    contact_email: Option<String>,
}

impl Manufacturer {
    /// 新しいメーカーを作成（バリデーション付き）
    pub fn new(
        name: String,
        website: Option<String>,
        contact_email: Option<String>,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: 名前は必須で、空文字列は不可
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Manufacturer name cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: 名前は200文字以内
        if name.len() > 200 {
            return Err(DomainError::ValidationError(
                "Manufacturer name must be 200 characters or less".to_string(),
            ));
        }

        // メールアドレスの簡易バリデーション
        if let Some(ref email) = contact_email
            && !email.trim().is_empty()
            && !is_valid_email(email)
        {
            return Err(DomainError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        Ok(Self {
            id: None,
            name: name.trim().to_string(),
            website: website
                .map(|w| w.trim().to_string())
                .filter(|w| !w.is_empty()),
            contact_email: contact_email
                .map(|e| e.trim().to_string())
                .filter(|e| !e.is_empty()),
        })
    }

    /// 既存のメーカーを再構築（DB から取得した場合など）
    pub fn reconstruct(
        id: i32,
        name: String,
        website: Option<String>,
        contact_email: Option<String>,
    ) -> Self {
        Self {
            id: Some(id),
            name,
            website,
            contact_email,
        }
    }

    /// メーカーの詳細を更新（バリデーション付き）
    pub fn update_details(
        &mut self,
        name: Option<String>,
        website: Option<String>,
        contact_email: Option<String>,
    ) -> Result<(), DomainError> {
        // 名前更新
        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(DomainError::ValidationError(
                    "Manufacturer name cannot be empty".to_string(),
                ));
            }
            if new_name.len() > 200 {
                return Err(DomainError::ValidationError(
                    "Manufacturer name must be 200 characters or less".to_string(),
                ));
            }
            self.name = new_name.trim().to_string();
        }

        // ウェブサイト更新
        if let Some(new_website) = website {
            self.website = if new_website.trim().is_empty() {
                None
            } else {
                Some(new_website.trim().to_string())
            };
        }

        // メール更新
        if let Some(new_email) = contact_email {
            if !new_email.trim().is_empty() && !is_valid_email(&new_email) {
                return Err(DomainError::ValidationError(
                    "Invalid email format".to_string(),
                ));
            }
            self.contact_email = if new_email.trim().is_empty() {
                None
            } else {
                Some(new_email.trim().to_string())
            };
        }

        Ok(())
    }

    // Getters
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn website(&self) -> Option<&str> {
        self.website.as_deref()
    }

    pub fn contact_email(&self) -> Option<&str> {
        self.contact_email.as_deref()
    }

    // ID をセット（リポジトリで保存後に使用）
    pub(crate) fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }
}

// 簡易的なメールアドレスバリデーション
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_manufacturer() {
        let manufacturer = Manufacturer::new(
            "Apple".to_string(),
            Some("https://www.apple.com".to_string()),
            Some("contact@apple.com".to_string()),
        );
        assert!(manufacturer.is_ok());
    }

    #[test]
    fn test_empty_name_fails() {
        let manufacturer = Manufacturer::new("".to_string(), None, None);
        assert!(manufacturer.is_err());
    }

    #[test]
    fn test_long_name_fails() {
        let long_name = "a".repeat(201);
        let manufacturer = Manufacturer::new(long_name, None, None);
        assert!(manufacturer.is_err());
    }

    #[test]
    fn test_invalid_email_fails() {
        let manufacturer =
            Manufacturer::new("Test".to_string(), None, Some("invalid-email".to_string()));
        assert!(manufacturer.is_err());
    }
}
