# LifeBook アーキテクチャドキュメント

## 概要

LifeBookは、クリーンアーキテクチャとドメイン駆動設計（DDD）の原則に基づいて設計された図書管理アプリケーションです。

## 設計原則

### 1. クリーンアーキテクチャ

依存関係の方向：

```
Presentation → Application → Domain ← Infrastructure
```

- **Domain層**: ビジネスルールとエンティティ（依存なし）
- **Application層**: ユースケースの調整（Domainに依存）
- **Infrastructure層**: 技術的な実装（Domainに依存）
- **Presentation層**: UIとAPI（Application/Domainに依存）

### 2. 境界づけられたコンテキスト（Bounded Context）

関連するドメインモデルを`modules`配下のサブドメインとしてグルーピングします。

- **library**: 図書管理コンテキスト
- **shared**: 全コンテキスト共通の要素

### 3. レイヤー単位の分割

各境界づけられたコンテキスト内部は、レイヤー単位で整理します。

## ディレクトリ構造

```
src-tauri/src/
├── modules/                          # ドメインモジュール
│   ├── library/                      # 図書管理の境界づけられたコンテキスト
│   │   ├── domain/                   # Domain層
│   │   │   ├── entities/             # エンティティ
│   │   │   │   └── book.rs           # Book Entity
│   │   │   └── repositories/         # リポジトリインターフェース
│   │   │       └── book.rs           # BookRepository trait
│   │   ├── application/              # Application層
│   │   │   ├── dto/                  # データ転送オブジェクト
│   │   │   │   └── book.rs           # BookDto
│   │   │   └── services/             # アプリケーションサービス
│   │   │       └── book.rs           # BookService
│   │   └── infrastructure/           # Infrastructure層
│   │       └── repositories/         # Repository実装
│   │           └── book.rs           # BookRepositoryImpl
│   └── shared/                       # 全コンテキスト共通
│       ├── domain/
│       │   └── errors.rs             # 共通ドメインエラー
│       └── application/
│           └── errors.rs             # 共通アプリケーションエラー
│
├── infrastructure/                   # 技術的詳細（コンテキスト外）
│   └── models/                       # SeaORM Models（全コンテキスト共有）
│       └── book.rs                   # Book Model + Relation
│
├── presentation/                     # Presentation層
│   ├── schema.rs                     # GraphQLスキーマ統合
│   └── library/                      # libraryコンテキストのAPI
│       ├── queries/
│       │   └── book.rs               # Book Query
│       └── mutations/
│           └── book.rs               # Book Mutation
│
├── app_state.rs                      # 依存性注入コンテナ
├── database.rs                       # データベース接続
├── migration.rs                      # DBマイグレーション
├── lib.rs                            # ライブラリエントリーポイント
└── main.rs                           # アプリケーションエントリーポイント
```

## 各レイヤーの責務

### Domain層（`modules/{context}/domain/`）

**責務**: ビジネスロジックとドメインモデル

- **entities/**: ビジネスルールを持つエンティティ
  - バリデーション
  - 不変条件の保護
  - ドメインロジック
- **repositories/**: リポジトリのインターフェース（trait）
  - 永続化の抽象化

**依存関係**: なし（他のレイヤーに依存しない）

**実装例**:

```rust
// modules/library/domain/entities/book.rs
pub struct Book {
    id: Option<i32>,
    title: String,
    // ...
}

impl Book {
    pub fn new(...) -> Result<Self, DomainError> {
        // バリデーション
        if title.trim().is_empty() {
            return Err(DomainError::ValidationError("Title cannot be empty"));
        }
        Ok(Self { ... })
    }
}
```

### Application層（`modules/{context}/application/`）

**責務**: ユースケースの調整

- **dto/**: データ転送オブジェクト
- **services/**: アプリケーションサービス
  - ドメインエンティティの協調
  - トランザクション管理
  - DTOへの変換

**依存関係**: Domain層のみ

**実装例**:

```rust
// modules/library/application/services/book.rs
pub struct BookService {
    repository: Arc<dyn BookRepository>,
}

impl BookService {
    pub async fn create_book(...) -> Result<BookDto, ApplicationError> {
        // 1. ドメインエンティティ作成
        let book = Book::new(...)?;
        // 2. 永続化
        let saved = self.repository.save(book).await?;
        // 3. DTOに変換
        Ok(BookDto::from(saved))
    }
}
```

### Infrastructure層

Infrastructure層は2つの場所に分かれています：

#### 1. コンテキスト内（`modules/{context}/infrastructure/`）

**責務**: そのコンテキストのRepository実装

- **repositories/**: リポジトリの実装
  - ドメインモデル ↔ DBモデルの変換
  - DB操作

**依存関係**: 自コンテキストのDomain層と、共有`infrastructure/models/`

**実装例**:
```rust
// modules/library/infrastructure/repositories/book.rs
use crate::infrastructure::models::book;  // 共有Modelを参照

pub struct BookRepositoryImpl {
    db: DatabaseConnection,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn save(&self, book: Book) -> Result<Book, DomainError> {
        // ドメインモデル → DBモデルの変換
        let active_model = Self::domain_to_active_model(&book);
        // DB操作
        let result = active_model.insert(&self.db).await?;
        // DBモデル → ドメインモデルの変換
        Ok(Self::db_to_domain(result))
    }
}
```

#### 2. コンテキスト外（`infrastructure/`）

**責務**: SeaORM Model（DBスキーマ）の定義

- **models/**: SeaORM Model
  - テーブル定義
  - リレーション定義

**なぜModelsはコンテキスト外か**:
- SeaORMのRelation定義では、同じディレクトリ内の方が参照が容易
- DBスキーマは技術的詳細で、複数のコンテキストから参照される
- Repository実装はコンテキスト内にあるため、ドメインの独立性は保たれる

**実装例**:
```rust
// infrastructure/models/book.rs
use sea_orm::entity::prelude::*;
use super::author;  // 同じディレクトリ内で簡単に参照

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub author_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "author::Entity", from = "Column::AuthorId", to = "author::Column::Id")]
    Author,
}
```

### Presentation層（`presentation/`）

**責務**: API層（薄いアダプター）

- **schema.rs**: GraphQLスキーマの統合
- **{context}/queries/**: GraphQL Query
- **{context}/mutations/**: GraphQL Mutation

**依存関係**: Application層とDomain層

**実装例**:

```rust
// presentation/library/queries/book.rs
#[Object]
impl BookQuery {
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<BookDto>> {
        let app_state = ctx.data::<AppState>()?;
        // Application Serviceに委譲
        app_state.book_service.get_all_books().await
            .map_err(|e| Error::new(e.to_string()))
    }
}
```

## 新しい機能の追加方法

### 1. 同じコンテキスト内に新しいエンティティを追加

例: `library`コンテキストに`Author`を追加

```
modules/library/
  ├── domain/
  │   ├── entities/
  │   │   ├── book.rs
  │   │   └── author.rs          # 追加
  │   └── repositories/
  │       ├── book.rs
  │       └── author.rs           # 追加
  ├── application/
  │   ├── dto/
  │   │   ├── book.rs
  │   │   └── author.rs           # 追加
  │   └── services/
  │       ├── book.rs
  │       └── author.rs           # 追加
  └── infrastructure/
      └── repositories/
          ├── book.rs
          └── author.rs           # 追加

infrastructure/models/
  ├── book.rs
  └── author.rs                   # 追加（コンテキスト外）
```

**手順**:
1. `domain/entities/author.rs` でエンティティとビジネスルールを定義
2. `domain/repositories/author.rs` でリポジトリインターフェースを定義
3. `application/dto/author.rs` でDTOを定義
4. `application/services/author.rs` でサービスを実装
5. `infrastructure/models/author.rs` でSeaORM Modelを定義（コンテキスト外）
6. `infrastructure/repositories/author.rs` でリポジトリ実装（コンテキスト内）
7. `presentation/library/queries/author.rs` でクエリを定義
8. `presentation/library/mutations/author.rs` でミューテーションを定義
9. 各`mod.rs`にモジュール宣言を追加
10. `app_state.rs`でサービスを登録

### 2. 複数エンティティの連携（Coordination）

例: `BookWithAuthor`として本と著者を一緒に取得

**オプションA**: Presentation層で結合（簡単なケース）

```rust
// presentation/library/queries/book_with_author.rs
async fn book_with_author(...) -> Result<BookWithAuthorDto> {
    let book = app_state.book_service.get_book(id).await?;
    let author = app_state.author_service.get_author(book.author_id).await?;
    Ok(BookWithAuthorDto { book, author })
}
```

**オプションB**: Coordination Serviceを作成（複雑なビジネスロジックがある場合）

```
modules/library/application/services/
  ├── book.rs
  ├── author.rs
  └── book_with_author.rs       # 調整サービス
```

```rust
// modules/library/application/services/book_with_author.rs
pub struct BookWithAuthorService {
    book_service: Arc<BookService>,
    author_service: Arc<AuthorService>,
}

impl BookWithAuthorService {
    pub async fn get_book_with_author(...) -> Result<BookWithAuthorDto> {
        // 複数のサービスを調整
    }
}
```

### 3. 新しい境界づけられたコンテキストの追加

例: ユーザー管理機能を追加

```
modules/
  ├── library/          # 既存
  ├── user/             # 新規追加
  │   ├── domain/
  │   │   ├── entities/
  │   │   └── repositories/
  │   ├── application/
  │   │   ├── dto/
  │   │   └── services/
  │   └── infrastructure/
  │       └── repositories/
  └── shared/

infrastructure/models/
  ├── book.rs           # 既存
  └── user.rs           # 追加（コンテキスト外）
```

## 依存性注入

`app_state.rs`で依存関係を構築します。

```rust
use crate::modules::library::{
    application::services::book::BookService,
    infrastructure::repositories::book::BookRepositoryImpl,
};

pub struct AppState {
    pub book_service: Arc<BookService>,
    // 他のサービスを追加
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        // 1. Repository実装を作成（コンテキスト内から）
        let book_repo = Arc::new(BookRepositoryImpl::new(db.clone()));
        
        // 2. Serviceを作成（Repositoryを注入）
        let book_service = Arc::new(BookService::new(book_repo));
        
        Self { book_service }
    }
}
```

## テスト戦略

### Domain層のテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_book() {
        let book = Book::new("Title".to_string(), ...);
        assert!(book.is_ok());
    }
}
```

### Application層のテスト

- モックリポジトリを使用してサービスをテスト

### Integration テスト

- 実際のDBを使用してエンドツーエンドでテスト

## ベストプラクティス

### ✅ やること

1. **ビジネスルールはDomain層に**
   - バリデーションはエンティティに
2. **薄いPresentation層を保つ**
   - Application Serviceへの委譲のみ
3. **Repository経由でのみDB操作**
   - 直接SeaORMを使わない
4. **DTOで境界を明確に**
   - ドメインモデルを直接返さない
5. **エラーは各層で適切に変換**
   - DomainError → ApplicationError → GraphQL Error

### ❌ 避けること

1. **Presentation層でビジネスロジック**
   - Application/Domain層に移動
2. **Domain層が他のレイヤーに依存**
   - 依存の方向を逆転させる
3. **複数のコンテキストが密結合**
   - Coordinationサービスで調整
4. **大きすぎるファイル**
   - 適切に分割する

## まとめ

この設計により：

- ✅ **拡張性**: 新機能の追加が容易
- ✅ **保守性**: 各レイヤーの責務が明確
- ✅ **テスタビリティ**: 依存性注入でテストが容易
- ✅ **再利用性**: モジュール化されたコード
- ✅ **チーム開発**: コンフリクトが起きにくい

質問や改善提案があれば、このドキュメントを更新してください。
