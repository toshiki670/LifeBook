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

### 4. モジュール可視性制御

各コンテキストのlib.rsでは`pub(crate)`を使用して内部実装を隠蔽します：

```rust
// contexts/{context}/src/lib.rs
pub(crate) mod application;     // クレート内のみ
pub(crate) mod domain;          // クレート内のみ
pub(crate) mod infrastructure;  // クレート内のみ
pub(crate) mod presentation;    // クレート内のみ

// Public API - re-exportのみ公開
pub use presentation::graphql::{Query, Mutation};
pub use presentation::integration::build_service;
```

これにより：

- 外部からは re-export されたもののみアクセス可能
- 内部実装の変更が外部に影響しない
- カプセル化が強化される

### 5. レイヤー単位の分割

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
│   ├── library/                      # 図書管理コンテキストCrate（DBベース）
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
│   ├── settings/                     # 設定管理コンテキストCrate（ファイルシステムベース）
│   │   ├── src/
│   │   │   ├── domain/               # Domain層
│   │   │   │   ├── entities/         # エンティティ
│   │   │   │   │   ├── settings.rs   # Settings Entity
│   │   │   │   │   ├── general.rs    # GeneralSettings
│   │   │   │   │   ├── appearance.rs # AppearanceSettings
│   │   │   │   │   └── database.rs   # DatabaseSettings
│   │   │   │   ├── value_objects/    # 値オブジェクト
│   │   │   │   │   ├── theme.rs      # Theme enum
│   │   │   │   │   └── language.rs   # Language enum
│   │   │   │   ├── repositories/     # リポジトリインターフェース
│   │   │   │   │   └── settings.rs   # SettingsRepository trait
│   │   │   │   ├── errors.rs         # DomainError
│   │   │   │   └── domain.rs         # Domainモジュール定義
│   │   │   ├── application/          # Application層
│   │   │   │   ├── dto/              # データ転送オブジェクト
│   │   │   │   │   ├── general.rs    # GeneralSettingsDto
│   │   │   │   │   ├── appearance.rs # AppearanceSettingsDto
│   │   │   │   │   └── database.rs   # DatabaseSettingsDto
│   │   │   │   ├── services/         # アプリケーションサービス
│   │   │   │   │   └── settings_service.rs # SettingsService
│   │   │   │   ├── errors.rs         # ApplicationError
│   │   │   │   └── application.rs    # Applicationモジュール定義
│   │   │   ├── infrastructure/       # Infrastructure層
│   │   │   │   ├── repositories/     # Repository実装
│   │   │   │   │   └── settings.rs   # SettingsRepositoryImpl（ファイルシステム）
│   │   │   │   └── infrastructure.rs # Infrastructureモジュール定義
│   │   │   ├── presentation/         # Presentation層（GraphQL）
│   │   │   │   ├── graphql/          # GraphQL API
│   │   │   │   │   ├── queries/
│   │   │   │   │   │   └── settings.rs # Settings Query
│   │   │   │   │   ├── mutations/
│   │   │   │   │   │   └── settings.rs # Settings Mutation
│   │   │   │   │   ├── error_ext.rs  # GraphQLエラー変換
│   │   │   │   │   └── graphql.rs    # GraphQLモジュール定義
│   │   │   │   ├── integration.rs    # 統合ヘルパー関数
│   │   │   │   └── presentation.rs   # Presentationモジュール定義
│   │   │   └── lib.rs                # Settingsのエントリーポイント
│   │   └── Cargo.toml                # Settings crateの依存関係
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
- **value_objects/**: 値オブジェクト（不変で交換可能）
  - 型安全性の向上
  - ビジネス概念の明示
  - バリデーションロジックのカプセル化
- **repositories/**: リポジトリのインターフェース（trait）
  - 永続化の抽象化
- **errors.rs**: コンテキスト固有のドメインエラー
  - `DomainError`（接頭辞なし）

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
            return Err(DomainError::ValidationError("Title cannot be empty".to_string()));
        }
        Ok(Self { ... })
    }
}
```

**Value Objectsの実装例**:

```rust
// contexts/settings/src/domain/value_objects/theme.rs
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, AsRefStr, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for Theme {
    fn default() -> Self {
        Self::System
    }
}
```

Value Objectsの利点：

- 型安全性：`String`ではなく`Theme`型を使用
- バリデーション：不正な値を防ぐ
- ビジネス概念の明示：コードの可読性向上
- 変更に強い：Themeの定義変更が一箇所で済む

### Application層（`contexts/{context}/src/application/`）

**責務**: ユースケースの調整

- **dto/**: データ転送オブジェクト
- **services/**: アプリケーションサービス
  - ドメインエンティティの協調
  - トランザクション管理
  - DTOへの変換
- **errors.rs**: コンテキスト固有のアプリケーションエラー
  - `ApplicationError`（接頭辞なし）
  - DomainErrorをラップ

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

**エラー型の実装例**:

```rust
// contexts/settings/src/application/errors.rs
use crate::domain::errors::DomainError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Invalid language code: {0}")]
    InvalidLanguage(String),

    #[error("Invalid theme: {0}")]
    InvalidTheme(String),

    #[error("Invalid database directory: {0}")]
    InvalidDatabaseDirectory(String),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
}
```

エラー型の命名規則：

- **コンテキスト固有**: 各コンテキストが独自の`DomainError`, `ApplicationError`を持つ
- **接頭辞なし**: `pub(crate)`で隠蔽されているため、衝突の心配なし
- **sharedとの一貫性**: 将来sharedが廃止されても、命名規則は統一

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
// contexts/settings/src/presentation/graphql/queries/settings.rs
use crate::presentation::graphql::to_graphql_error;

#[Object]
impl SettingsQuery {
    async fn general_settings(&self, ctx: &Context<'_>) -> Result<GeneralSettingsDto> {
        let settings_service = ctx.data::<Arc<SettingsService>>()?;
        // Application Serviceに委譲
        settings_service
            .get_general_settings()
            .await
            .map_err(to_graphql_error)  // エラーコード付きGraphQLエラーに変換
    }
}
```

**GraphQLエラー変換**:

```rust
// contexts/settings/src/presentation/graphql/error_ext.rs
use crate::application::errors::ApplicationError;
use async_graphql::{Error, ErrorExtensions};

pub fn to_graphql_error(e: ApplicationError) -> Error {
    match e {
        ApplicationError::InvalidLanguage(msg) => Error::new(msg).extend_with(|_, ext| {
            ext.set("code", "INVALID_LANGUAGE");
        }),
        ApplicationError::InvalidTheme(msg) => Error::new(msg).extend_with(|_, ext| {
            ext.set("code", "INVALID_THEME");
        }),
        ApplicationError::Domain(e) => {
            Error::new(format!("Domain error: {}", e)).extend_with(|_, ext| {
                ext.set("code", "DOMAIN_ERROR");
            })
        }
    }
}
```

エラーコードを付与することで、フロントエンドでエラーの種類を判別しやすくなります。

#### 2. メインアプリ（`lifebook/src/graphql_schema.rs`）

**責務**: GraphQLスキーマの統合のみ

- **graphql_schema.rs**: 各コンテキストから提供されるQuery/Mutationを統合

**実装例**:

```rust
// lifebook/src/graphql_schema.rs
use library::{BookQuery, BookMutation};
use settings::{SettingsQuery, SettingsMutation};

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn library(&self) -> BookQuery {
        BookQuery
    }

    async fn settings(&self) -> SettingsQuery {
        SettingsQuery
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn library(&self) -> BookMutation {
        BookMutation
    }

    async fn settings(&self) -> SettingsMutation {
        SettingsMutation
    }
}

pub fn build_schema(app_state: AppState) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(app_state.book_service)
        .data(app_state.settings_service)
        .finish()
}
```

#### 3. Presentation層の統合ヘルパー（`contexts/{context}/src/presentation/integration.rs`）

**責務**: 依存性注入の簡素化

外部からの依存性注入を簡単にするため、統合ヘルパー関数を提供します。

**実装例**:

```rust
// contexts/settings/src/presentation/integration.rs
use crate::application::services::SettingsService;
use crate::infrastructure::repositories::SettingsRepositoryImpl;
use std::path::PathBuf;
use std::sync::Arc;

pub fn build_settings_service(
    config_dir: PathBuf,
    default_db_dir: PathBuf,
) -> Arc<SettingsService> {
    let settings_repo = Arc::new(SettingsRepositoryImpl::new(config_dir, default_db_dir));
    Arc::new(SettingsService::new(settings_repo))
}
```

これにより、lifebookアプリは内部実装を知らずにサービスを構築できます：

```rust
// lifebook/src/app_state.rs
use settings::build_settings_service;

let settings_service = build_settings_service(config_dir, default_db_dir);
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

各コンテキストは統合ヘルパー関数を提供し、lifebookアプリは内部実装を知らずにサービスを構築できます。

```rust
use library::{BookRepositoryImpl, BookService};
use settings::build_settings_service;
use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use std::sync::Arc;

pub struct AppState {
    pub book_service: Arc<BookService>,
    pub settings_service: Arc<SettingsService>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config_dir: PathBuf, default_db_dir: PathBuf) -> Self {
        // Library Context（従来の方法）
        let book_repo = Arc::new(BookRepositoryImpl::new(db.clone()));
        let book_service = Arc::new(BookService::new(book_repo));

        // Settings Context（統合ヘルパー関数）
        let settings_service = build_settings_service(config_dir, default_db_dir);

        Self {
            book_service,
            settings_service,
        }
    }
}
```

## Crate間の依存関係

```
lifebook (メインアプリ)
  ├─> library (図書管理コンテキスト)
  │     ├─> shared (共通要素) ※将来廃止予定
  │     └─> entity (DBエンティティ)
  ├─> settings (設定管理コンテキスト)
  │     └─> (依存なし - 完全独立)
  ├─> shared ※将来廃止予定
  ├─> entity
  └─> migration

migration
  └─> entity
```

各コンテキストCrateは：

- **完全に独立**: 他のコンテキストに依存しない（疎結合）
- **必要に応じて共有crateを参照**:
  - `entity`: SeaORM Entity（DBベースのコンテキスト）
  - `shared`: 共通エラー型（※将来廃止予定）
- **pub(crate)で内部実装を隠蔽**: 外部からはre-exportのみアクセス可能

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
   - Value Objectsで型安全性を高める
2. **薄いPresentation層を保つ**
   - Application Serviceへの委譲のみ
   - エラー変換とエラーコード付与
3. **Repository経由でのみDB操作**
   - 直接SeaORMを使わない
4. **DTOで境界を明確に**
   - ドメインモデルを直接返さない
5. **エラーは各層で適切に変換**
   - DomainError → ApplicationError → GraphQL Error（エラーコード付き）
6. **pub(crate)で内部実装を隠蔽**
   - re-exportのみを公開APIとする
7. **統合ヘルパー関数を提供**
   - Presentation層で依存性注入を簡素化
8. **コンテキスト固有のエラー型を定義**
   - 接頭辞なしの`DomainError`, `ApplicationError`

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

---

## Feature: Books（フロントエンド）

アプリの Books 機能（作成/更新/一覧/詳細）のUIとデータフロー設計。React/Remix + Apollo Client（Tauri Link）を使用します。

### ディレクトリ

```
src/features/books/
  create/      # 作成ページ(+actions)
  update/      # 更新ページ(+loader/actions)
  detail/      # 詳細取得APIなど
  list/        # 一覧UI
  shared/      # 共通フォーム・型・ユーティリティ
  graphql/     # GraphQLドキュメント/生成物
  layouts/     # レイアウト
```

### 共通フォーム

- `shared/components/BookForm.tsx` に Create/Update 共通の入力UIを集約
- プロパティ:
  - `title`, `initialValues`, `isSubmitting`, `submitLabel`, `submittingLabel`, `cancelTo`
- 呼び出し元ルートが `<Form method="post">` を内部で描画

### フォーム入力 → 値変換

- `shared/utils/parseBookFormData.ts`: `FormData` を `BookFormValues` に変換
- 文字列空値は `undefined`、`publishedYear` は数値に変換

### ルーティング/データフロー

- Create: 画面 → `clientAction` → `createBook` Mutation → 成功時 `/books` に戻る
- Update: `clientLoader` で対象取得 → 画面（初期値）→ `clientAction` → `updateBook` → 成功時 `/books/:id`
- 送信中状態は `useNavigation().state === "submitting"`

### GraphQL/Apollo

- 生成済みドキュメントを利用（`books/graphql`）
- `src/lib/apollo-client.ts`
  - クライアント: Tauri の `invoke` を用いる `ApolloLink`
  - `defaultOptions.errorPolicy = "all"`
  - SSR が必要な場合は `HttpLink` + `GRAPHQL_ENDPOINT` による環境別生成を検討

### 拡張指針

1. フィールド追加時は `BookFormValues` → `BookForm` → `parseBookFormData` の順に更新
2. バリデーションは段階的に（クライアント最小限 → GraphQLエラーをUIへ）
3. ナビゲーションは成功時 `navigate`、キャンセルは `cancelTo` に委譲

関連ファイル:

- `src/features/books/shared/types.ts`
- `src/features/books/shared/components/BookForm.tsx`
- `src/features/books/shared/utils/parseBookFormData.ts`
- `src/features/books/create/page.tsx`
- `src/features/books/update/page.tsx`
