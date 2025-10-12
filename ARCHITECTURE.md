# LifeBook アーキテクチャドキュメント

## 概要

LifeBookは、クリーンアーキテクチャとドメイン駆動設計（DDD）の原則に基づいて設計された図書管理アプリケーションです。

各境界づけられたコンテキスト（Bounded Context）は独立したRust Crateとして実装され、明確な境界と依存関係を持ちます。

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

関連するドメインモデルを独立したCrateとして`contexts/`配下に配置します。

- **library**: 図書管理コンテキスト（Crate）
- **shared**: 全コンテキスト共通の要素（Crate）

各コンテキストは独立してビルド・テスト可能で、明確なAPI境界を持ちます。

### 3. レイヤー単位の分割

各境界づけられたコンテキスト内部は、レイヤー単位で整理します。

## ディレクトリ構造

```
src-tauri/
├── lifebook/                         # メインアプリケーション（Tauri App）
│   ├── src/
│   │   ├── graphql_schema.rs         # GraphQLスキーマ統合
│   │   ├── app_state.rs              # 依存性注入コンテナ
│   │   ├── database.rs               # データベース接続
│   │   ├── lib.rs                    # ライブラリエントリーポイント
│   │   └── main.rs                   # アプリケーションエントリーポイント
│   └── Cargo.toml                    # メインアプリの依存関係
│
├── contexts/                         # 境界づけられたコンテキスト（Crates）
│   ├── library/                      # 図書管理コンテキストCrate
│   │   ├── src/
│   │   │   ├── domain/               # Domain層
│   │   │   │   ├── entities/         # エンティティ
│   │   │   │   │   └── book.rs       # Book Entity
│   │   │   │   ├── repositories/     # リポジトリインターフェース
│   │   │   │   │   └── book.rs       # BookRepository trait
│   │   │   │   └── domain.rs         # Domainモジュール定義
│   │   │   ├── application/          # Application層
│   │   │   │   ├── dto/              # データ転送オブジェクト
│   │   │   │   │   └── book.rs       # BookDto
│   │   │   │   ├── services/         # アプリケーションサービス
│   │   │   │   │   └── book.rs       # BookService
│   │   │   │   └── application.rs    # Applicationモジュール定義
│   │   │   ├── infrastructure/       # Infrastructure層
│   │   │   │   ├── repositories/     # Repository実装
│   │   │   │   │   └── book.rs       # BookRepositoryImpl
│   │   │   │   └── infrastructure.rs # Infrastructureモジュール定義
│   │   │   ├── presentation/         # Presentation層（GraphQL）
│   │   │   │   ├── graphql/          # GraphQL API
│   │   │   │   │   ├── queries/
│   │   │   │   │   │   └── book.rs   # Book Query
│   │   │   │   │   ├── mutations/
│   │   │   │   │   │   └── book.rs   # Book Mutation
│   │   │   │   │   └── graphql.rs    # GraphQLモジュール定義
│   │   │   │   └── presentation.rs   # Presentationモジュール定義
│   │   │   └── lib.rs                # Libraryのエントリーポイント
│   │   └── Cargo.toml                # Library crateの依存関係
│   │
│   └── shared/                       # 共通コンテキストCrate
│       ├── src/
│       │   ├── domain/
│       │   │   ├── errors.rs         # 共通ドメインエラー
│       │   │   └── domain.rs         # Domainモジュール定義
│       │   ├── application/
│       │   │   ├── errors.rs         # 共通アプリケーションエラー
│       │   │   └── application.rs    # Applicationモジュール定義
│       │   └── lib.rs                # Sharedのエントリーポイント
│       └── Cargo.toml                # Shared crateの依存関係
│
├── entity/                           # SeaORM Entities（全コンテキスト共有）
│   ├── src/
│   │   ├── book.rs                   # Book Model + Relation
│   │   └── lib.rs
│   └── Cargo.toml
│
├── migration/                        # DBマイグレーション
│   ├── src/
│   │   ├── m20250108_000001_create_book_table.rs
│   │   ├── lib.rs
│   │   └── main.rs
│   └── Cargo.toml
│
└── Cargo.toml                        # Workspaceの定義
```

## 各レイヤーの責務

### Domain層（`contexts/{context}/src/domain/`）

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
// contexts/library/src/domain/entities/book.rs
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

### Application層（`contexts/{context}/src/application/`）

**責務**: ユースケースの調整

- **dto/**: データ転送オブジェクト
- **services/**: アプリケーションサービス
  - ドメインエンティティの協調
  - トランザクション管理
  - DTOへの変換

**依存関係**: Domain層のみ

**実装例**:

```rust
// contexts/library/src/application/services/book.rs
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

#### 1. コンテキスト内（`contexts/{context}/src/infrastructure/`）

**責務**: そのコンテキストのRepository実装

- **repositories/**: リポジトリの実装
  - ドメインモデル ↔ DBモデルの変換
  - DB操作

**依存関係**: 自コンテキストのDomain層と、共有`entity` crate

**実装例**:

```rust
// contexts/library/src/infrastructure/repositories/book.rs
use entity::book;  // 共有Entity crateを参照

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

#### 2. コンテキスト外（`entity/` crate）

**責務**: SeaORM Entity（DBスキーマ）の定義

- **entity/**: SeaORM Entity
  - テーブル定義
  - リレーション定義

**なぜEntityはコンテキスト外か**:

- SeaORMのRelation定義では、同じcrate内の方が参照が容易
- DBスキーマは技術的詳細で、複数のコンテキストから参照される
- Repository実装はコンテキスト内にあるため、ドメインの独立性は保たれる

**実装例**:

```rust
// entity/src/book.rs
use sea_orm::entity::prelude::*;
use super::author;  // 同じcrate内で簡単に参照

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

### Presentation層

Presentation層は2つの場所に分かれています：

#### 1. コンテキスト内（`contexts/{context}/src/presentation/graphql/`）

**責務**: 各コンテキストのGraphQL API定義

- **queries/**: GraphQL Query
- **mutations/**: GraphQL Mutation

**実装例**:

```rust
// contexts/library/src/presentation/graphql/queries/book.rs
#[Object]
impl BookQuery {
    async fn books(&self, ctx: &Context<'_>) -> Result<Vec<BookDto>> {
        let book_service = ctx.data::<Arc<BookService>>()?;
        // Application Serviceに委譲
        book_service.get_all_books().await
            .map_err(|e| Error::new(e.to_string()))
    }
}
```

#### 2. メインアプリ（`lifebook/src/graphql_schema.rs`）

**責務**: GraphQLスキーマの統合のみ

- **graphql_schema.rs**: 各コンテキストから提供されるQuery/Mutationを`MergedObject`で統合

**実装例**:

```rust
// lifebook/src/graphql_schema.rs
use library::{BookQuery, BookMutation};

#[derive(MergedObject, Default)]
pub struct QueryRoot(BookQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(BookMutation);

pub fn build_schema(app_state: AppState) -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .data(app_state.book_service)
        .finish()
}
```

## 新しい機能の追加方法

### 1. 同じコンテキスト内に新しいエンティティを追加

例: `library`コンテキストに`Author`を追加

```
contexts/library/src/
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
  ├── infrastructure/
  │   └── repositories/
  │       ├── book.rs
  │       └── author.rs           # 追加
  └── presentation/graphql/
      ├── queries/
      │   ├── book.rs
      │   └── author.rs           # 追加
      └── mutations/
          ├── book.rs
          └── author.rs           # 追加

entity/src/
  ├── book.rs
  └── author.rs                   # 追加（コンテキスト外）
```

**手順**:

1. `contexts/library/src/domain/entities/author.rs` でエンティティとビジネスルールを定義
2. `contexts/library/src/domain/repositories/author.rs` でリポジトリインターフェースを定義
3. `contexts/library/src/application/dto/author.rs` でDTOを定義
4. `contexts/library/src/application/services/author.rs` でサービスを実装
5. `entity/src/author.rs` でSeaORM Entityを定義（コンテキスト外）
6. `contexts/library/src/infrastructure/repositories/author.rs` でリポジトリ実装（コンテキスト内）
7. `contexts/library/src/presentation/graphql/queries/author.rs` でクエリを定義
8. `contexts/library/src/presentation/graphql/mutations/author.rs` でミューテーションを定義
9. 各モジュールファイル（`entities.rs`, `repositories.rs`, `dto.rs`, `services.rs`等）にモジュール宣言を追加
10. `contexts/library/src/lib.rs` でエクスポート
11. `lifebook/src/app_state.rs`でサービスを登録
12. `lifebook/src/graphql_schema.rs`で`AuthorQuery`と`AuthorMutation`を`MergedObject`に追加

### 2. 複数エンティティの連携（Coordination）

例: `BookWithAuthor`として本と著者を一緒に取得

**オプションA**: Presentation層で結合（簡単なケース）

```rust
// contexts/library/src/presentation/graphql/queries/book_with_author.rs
async fn book_with_author(...) -> Result<BookWithAuthorDto> {
    let book_service = ctx.data::<Arc<BookService>>()?;
    let author_service = ctx.data::<Arc<AuthorService>>()?;

    let book = book_service.get_book(id).await?;
    let author = author_service.get_author(book.author_id).await?;
    Ok(BookWithAuthorDto { book, author })
}
```

**オプションB**: Coordination Serviceを作成（複雑なビジネスロジックがある場合）

```
contexts/library/src/application/services/
  ├── book.rs
  ├── author.rs
  └── book_with_author.rs       # 調整サービス
```

```rust
// contexts/library/src/application/services/book_with_author.rs
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
src-tauri/
  ├── contexts/
  │   ├── library/                    # 既存
  │   ├── user/                       # 新規追加
  │   │   ├── src/
  │   │   │   ├── domain/
  │   │   │   │   ├── entities/
  │   │   │   │   ├── repositories/
  │   │   │   │   └── domain.rs
  │   │   │   ├── application/
  │   │   │   │   ├── dto/
  │   │   │   │   ├── services/
  │   │   │   │   └── application.rs
  │   │   │   ├── infrastructure/
  │   │   │   │   ├── repositories/
  │   │   │   │   └── infrastructure.rs
  │   │   │   ├── presentation/
  │   │   │   │   ├── graphql/
  │   │   │   │   │   ├── queries/
  │   │   │   │   │   ├── mutations/
  │   │   │   │   │   └── graphql.rs
  │   │   │   │   └── presentation.rs
  │   │   │   └── lib.rs
  │   │   └── Cargo.toml
  │   └── shared/                     # 既存
  │
  └── entity/src/
      ├── book.rs                     # 既存
      └── user.rs                     # 追加（コンテキスト外）
```

**手順**:

1. `contexts/user/` ディレクトリを作成
2. `contexts/user/Cargo.toml` を作成（`shared`と`entity`に依存）
3. 各レイヤー（domain, application, infrastructure, presentation）を実装
4. `src-tauri/Cargo.toml` のworkspace membersに`contexts/user`を追加
5. `lifebook/Cargo.toml` の依存関係に`user = { path = "../contexts/user" }`を追加
6. `lifebook/src/app_state.rs` でuser contextのサービスを登録
7. `lifebook/src/graphql_schema.rs` で`UserQuery`と`UserMutation`を`MergedObject`に追加

## 依存性注入

`lifebook/src/app_state.rs`で依存関係を構築します。

```rust
use library::{BookRepositoryImpl, BookService};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct AppState {
    pub book_service: Arc<BookService>,
    // 他のサービスを追加
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        // 1. Repository実装を作成（library crateから）
        let book_repo = Arc::new(BookRepositoryImpl::new(db.clone()));

        // 2. Serviceを作成（Repositoryを注入）
        let book_service = Arc::new(BookService::new(book_repo));

        Self { book_service }
    }
}
```

## Crate間の依存関係

```
lifebook (メインアプリ)
  ├─> library (図書管理コンテキスト)
  │     ├─> shared (共通要素)
  │     └─> entity (DBエンティティ)
  ├─> shared
  ├─> entity
  └─> migration

migration
  └─> entity
```

各コンテキストCrateは：

- **shared** crateに依存（共通エラー型など）
- **entity** crateに依存（SeaORM Entity）
- 他のコンテキストには依存しない（疎結合）

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
