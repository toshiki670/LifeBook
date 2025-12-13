// Asset Domain - Category Entity

use crate::domain::errors::DomainError;

/// Category エンティティ（ビジネスルールを持つドメインモデル）
#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    id: Option<i32>,
    name: String,
    parent_id: Option<i32>,
    depth: i32,
}

impl Category {
    /// 新しいカテゴリを作成（バリデーション付き）
    pub fn new(name: String, parent_id: Option<i32>) -> Result<Self, DomainError> {
        // ビジネスルール: 名前は必須で、空文字列は不可
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Category name cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: 名前は100文字以内
        if name.len() > 100 {
            return Err(DomainError::ValidationError(
                "Category name must be 100 characters or less".to_string(),
            ));
        }

        Ok(Self {
            id: None,
            name: name.trim().to_string(),
            parent_id,
            depth: 0, // 作成時にリポジトリで計算される
        })
    }

    /// 既存のカテゴリを再構築（DB から取得した場合など）
    pub fn reconstruct(id: i32, name: String, parent_id: Option<i32>, depth: i32) -> Self {
        Self {
            id: Some(id),
            name,
            parent_id,
            depth,
        }
    }

    /// カテゴリ名を更新（バリデーション付き）
    pub fn update_name(&mut self, name: String) -> Result<(), DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError(
                "Category name cannot be empty".to_string(),
            ));
        }
        if name.len() > 100 {
            return Err(DomainError::ValidationError(
                "Category name must be 100 characters or less".to_string(),
            ));
        }
        self.name = name.trim().to_string();
        Ok(())
    }

    /// 親を変更（循環参照チェックは別途必要）
    pub fn change_parent(
        &mut self,
        new_parent_id: Option<i32>,
        new_depth: i32,
    ) -> Result<(), DomainError> {
        // 自分自身を親にできない
        if let Some(pid) = new_parent_id
            && let Some(self_id) = self.id
            && pid == self_id
        {
            return Err(DomainError::CannotSetSelfAsParent);
        }

        // depth の妥当性チェック
        if !(0..=10).contains(&new_depth) {
            return Err(DomainError::ValidationError(
                "Category depth must be between 0 and 10".to_string(),
            ));
        }

        self.parent_id = new_parent_id;
        self.depth = new_depth;
        Ok(())
    }

    /// depthを検証
    pub fn validate_max_depth(&self, max_depth: i32) -> Result<(), DomainError> {
        if self.depth >= max_depth {
            return Err(DomainError::ValidationError(format!(
                "Maximum depth of {} exceeded",
                max_depth
            )));
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

    pub fn parent_id(&self) -> Option<i32> {
        self.parent_id
    }

    pub fn depth(&self) -> i32 {
        self.depth
    }

    // Setters (リポジトリで使用)
    pub(crate) fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub(crate) fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_category() {
        let category = Category::new("家電".to_string(), None);
        assert!(category.is_ok());
    }

    #[test]
    fn test_empty_name_fails() {
        let category = Category::new("".to_string(), None);
        assert!(category.is_err());
    }

    #[test]
    fn test_long_name_fails() {
        let long_name = "a".repeat(101);
        let category = Category::new(long_name, None);
        assert!(category.is_err());
    }

    #[test]
    fn test_cannot_set_self_as_parent() {
        let mut category = Category::reconstruct(1, "Test".to_string(), None, 0);
        let result = category.change_parent(Some(1), 1);
        assert!(result.is_err());
    }
}
