# Tauri + SeaORM + GraphQL 統合ガイド

このプロジェクトは、TauriアプリケーションでSeaORMとGraphQLを統合する実装例です。

## 📋 概要

- **Tauri**: デスクトップアプリケーションフレームワーク
- **SeaORM**: Rustの非同期ORM
- **async-graphql**: Rust用GraphQLサーバーライブラリ
- **SQLite**: 組み込みデータベース

## 🏗️ アーキテクチャ

### Rustサイド (Backend)

```
src-tauri/src/
├── modules/                        # ドメインモジュール
│   ├── library/                    # 図書管理コンテキスト
│   │   ├── domain/                 # エンティティ、リポジトリtrait
│   │   ├── application/            # DTO、サービス
│   │   └── infrastructure/         # リポジトリ実装
│   └── shared/                     # 共通エラー
├── infrastructure/                 # 技術的詳細
│   └── models/                     # SeaORM Models
├── presentation/                   # GraphQL実装
│   ├── schema.rs                   # GraphQLスキーマ構築
│   └── library/
│       ├── queries/                # GraphQL Queries
│       └── mutations/              # GraphQL Mutations
├── app_state.rs                    # 依存性注入コンテナ
├── database.rs                     # データベース接続管理
├── migration.rs                    # DBマイグレーション
├── lib.rs                          # メインエントリーポイント
└── main.rs                         # バイナリエントリー
```

**注**: このプロジェクトはモジュラーアーキテクチャと境界づけられたコンテキスト（DDD）を採用しています。詳細は[ARCHITECTURE.md](./ARCHITECTURE.md)を参照してください。

### フロントエンド (React + TypeScript)

```
src/
├── lib/
│   └── graphql.ts      # GraphQLクライアントユーティリティ
└── routes/
    ├── home.tsx        # ホームページ
    └── books.tsx       # 本管理ページ（GraphQLデモ）
```

## 🔧 主要コンポーネント

### 1. データベース接続 (`database.rs`)

```rust
pub async fn init_database() -> Result<DatabaseConnection, DbErr> {
    let database_url = "sqlite://lifebook.db?mode=rwc";
    let db = Database::connect(database_url).await?;
    Ok(db)
}
```

### 2. マイグレーション (`migration.rs`)

このプロジェクトは[SeaORM公式ドキュメント](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/)に従って、`sea-orm-migration`を使用しています。

**重要**: このプロジェクトでは`mod.rs`を使わず、単一の`migration.rs`ファイルに全マイグレーションをまとめています。

#### `migration.rs` の全体構造

```rust
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250108_000001_create_book_table::Migration)]
    }
}

// Migration: Create Book Table
mod m20250108_000001_create_book_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Book::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Book::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Book::Title).string().not_null())
                        .col(ColumnDef::new(Book::Author).string())
                        .col(ColumnDef::new(Book::Description).string())
                        .col(ColumnDef::new(Book::PublishedYear).integer())
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(Book::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum Book {
        Table,
        Id,
        Title,
        Author,
        Description,
        PublishedYear,
    }
}
```

新しいマイグレーションを追加する場合は、同じファイル内にmodブロックを追加し、`Migrator::migrations()`に登録します。

マイグレーションはアプリケーション起動時に自動実行されます：

```rust
use sea_orm_migration::MigratorTrait;
migration::Migrator::up(&db, None).await?;
```

### 3. ドメインエンティティ (`modules/library/domain/entities/book.rs`)

ドメイン層でビジネスルールとバリデーションを定義します：

```rust
use crate::modules::shared::domain::errors::DomainError;

pub struct Book {
    id: Option<i32>,
    title: String,
    author: Option<String>,
    description: Option<String>,
    published_year: Option<i32>,
}

impl Book {
    pub fn new(title: String, ...) -> Result<Self, DomainError> {
        // ビジネスルールのバリデーション
        if title.trim().is_empty() {
            return Err(DomainError::ValidationError("Title cannot be empty".to_string()));
        }
        Ok(Self { ... })
    }
}
```

### 4. SeaORM Models (`infrastructure/models/book.rs`)

DBスキーマの定義（コンテキスト外で全コンテキスト共有）：

```rust
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub published_year: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

### 5. Repository実装 (`modules/library/infrastructure/repositories/book.rs`)

ドメインモデルとDBモデルの変換を担当：

```rust
use crate::infrastructure::models::book;
use crate::modules::library::domain::{entities::book::Book, repositories::book::BookRepository};

pub struct BookRepositoryImpl {
    db: DatabaseConnection,
}

impl BookRepositoryImpl {
    fn domain_to_active_model(book: &Book) -> book::ActiveModel { ... }
    fn db_to_domain(model: book::Model) -> Book { ... }
}
```

### 6. GraphQLスキーマ

#### コンテキスト単位のAPI設計

各コンテキストは独自のQuery/Mutationを提供し、メインアプリで統合されます。

```graphql
# Library Context
query {
  library {
    books {
      id
      title
    }
    book(id: 1) {
      id
      title
    }
  }
}

# Settings Context（セクション単位）
query {
  settings {
    generalSettings {
      language
    }
    appearanceSettings {
      theme
    }
    databaseSettings {
      databaseDirectory
    }
  }
}
```

#### Library Context（`presentation/library/`）

**Query**:

- `books`: すべての本を取得
- `book(id: Int!)`: IDで本を取得

**Mutation**:

- `createBook(...)`: 新しい本を作成
- `updateBook(...)`: 本を更新
- `deleteBook(id: Int!)`: 本を削除

#### Settings Context（`presentation/settings/`）

**Query**（セクション単位）:

- `generalSettings`: 一般設定を取得
- `appearanceSettings`: 表示設定を取得
- `databaseSettings`: データベース設定を取得

**Mutation**（セクション単位）:

- `updateGeneralSettings(language: String)`: 一般設定を更新
- `updateAppearanceSettings(theme: String)`: 表示設定を更新
- `updateDatabaseSettings(databaseDirectory: String)`: データベース設定を更新
- `resetAllSettings`: 全設定をリセット

### 7. Tauriコマンド (`lib.rs`)

```rust
#[tauri::command]
async fn execute_graphql(
    query: String,
    schema: tauri::State<'_, AppSchema>,
) -> Result<String, String> {
    let result = schema.execute(&query).await;
    Ok(serde_json::to_string(&result).map_err(|e| e.to_string())?)
}
```

フロントエンドからGraphQLクエリ文字列を受け取り、実行結果をJSON文字列として返します。

### 8. フロントエンドクライアント

#### Apollo Client (`lib/apollo-client.ts`)

Tauri invoke経由でGraphQLクエリを実行するカスタムApollo Clientを使用：

```typescript
import { ApolloClient, ApolloLink, InMemoryCache } from "@apollo/client"
import { invoke } from "@tauri-apps/api/core"

const tauriLink = new ApolloLink((operation) => {
  // Tauri invoke経由でGraphQLリクエストを実行
  return invoke("graphql_request", { request: JSON.stringify(request) })
})

export const apolloClient = new ApolloClient({
  link: tauriLink,
  cache: new InMemoryCache(),
})
```

#### GraphQL Code Generator

スキーマから自動生成された型安全なhooksを使用：

```bash
# スキーマをエクスポート
pnpm export-schema

# 型とhooksを生成
pnpm codegen
```

生成されたhooksの使用例：

```typescript
import { useGetBooksQuery, useCreateBookMutation } from "~/generated/graphql"

function BooksList() {
  const { data, loading, error } = useGetBooksQuery()
  const [createBook] = useCreateBookMutation()

  // 型安全にデータにアクセス
  const books = data?.library?.books || []
}

## 🚀 使い方

### 開発モードで実行

```bash
pnpm tauri dev
```

### ビルド

```bash
pnpm tauri build
```

### GraphQLクエリの例

#### すべての本を取得

```graphql
query {
  books {
    id
    title
    author
    description
    publishedYear
  }
}
```

#### 本を作成

```graphql
mutation {
  createBook(
    title: "The Rust Programming Language"
    author: "Steve Klabnik, Carol Nichols"
    publishedYear: 2018
  ) {
    id
    title
    author
  }
}
```

#### 本を更新

```graphql
mutation {
  updateBook(id: 1, description: "An excellent introduction to Rust") {
    id
    title
    description
  }
}
```

#### 本を削除

```graphql
mutation {
  deleteBook(id: 1)
}
```

## 💡 実装のポイント

### 1. 非同期処理の扱い

Tauriは非同期コマンドをサポートしているため、`async fn`を使用できます：

```rust
#[tauri::command]
async fn execute_graphql(...) -> Result<String, String> {
    // async-graphqlの非同期実行
}
```

### 2. 状態管理

GraphQLスキーマとデータベース接続は、Tauriの状態管理機能で管理：

```rust
.manage(schema)
.manage(db_state)
```

### 3. エラーハンドリング

#### Rust側: エラーコード付きGraphQLエラー

```rust
// contexts/settings/src/presentation/graphql/error_ext.rs
use crate::application::errors::ApplicationError;
use async_graphql::{Error, ErrorExtensions};

pub fn to_graphql_error(e: ApplicationError) -> Error {
    match e {
        ApplicationError::InvalidLanguage(msg) => {
            Error::new(msg).extend_with(|_, ext| {
                ext.set("code", "INVALID_LANGUAGE");
            })
        },
        ApplicationError::InvalidTheme(msg) => {
            Error::new(msg).extend_with(|_, ext| {
                ext.set("code", "INVALID_THEME");
            })
        },
        ApplicationError::Domain(e) => {
            Error::new(format!("Domain error: {}", e)).extend_with(|_, ext| {
                ext.set("code", "DOMAIN_ERROR");
            })
        }
    }
}
```

#### フロントエンド側: エラーコードによる分岐

GraphQLレスポンスには`data`と`errors`の両方が含まれる可能性があるため、
フロントエンド側で適切に処理：

```typescript
const response = await getBooks();
if (response.errors) {
  // エラーコードで分岐
  const errorCode = response.errors[0]?.extensions?.code;
  switch (errorCode) {
    case "INVALID_LANGUAGE":
      // 言語エラー処理
      break;
    case "INVALID_THEME":
      // テーマエラー処理
      break;
    case "DOMAIN_ERROR":
      // ドメインエラー処理
      break;
    default:
    // 一般エラー処理
  }
} else if (response.data) {
  // 成功時の処理
}
```

#### エラー変換の流れ

```
Domain Layer
  ↓ DomainError (ValidationError, IoError, etc.)
Application Layer
  ↓ ApplicationError (InvalidLanguage, InvalidTheme, Domain, etc.)
Presentation Layer
  ↓ GraphQL Error + Error Code (INVALID_LANGUAGE, INVALID_THEME, etc.)
Frontend
  ↓ User-friendly message
```

### 4. 型安全性

#### Rust側

- **SeaORM**: データベースモデルの型安全性
- **async-graphql**: GraphQLスキーマの型安全性
- **Value Objects**: ドメイン概念の型安全性

```rust
// Value Objectsの例
#[derive(Serialize, Deserialize, EnumString, AsRefStr, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum Theme {
    Light,
    Dark,
    System,
}

// GraphQLでの使用
#[Object]
impl SettingsQuery {
    async fn appearance_settings(&self) -> AppearanceSettingsDto {
        // Theme型で型安全に扱う
    }
}
```

#### TypeScript側

GraphQLレスポンスの型を定義：

```typescript
interface Book {
  id: number;
  title: string;
  author?: string;
}

interface GraphQLResponse<T> {
  data?: T;
  errors?: Array<{
    message: string;
    extensions?: {
      code?: string;
    };
  }>;
}
```

## 🔄 開発ワークフロー

### GraphQLスキーマの変更

1. Rust側でスキーマを変更（型、Query、Mutationなど）
2. スキーマをエクスポート: `pnpm export-schema`
3. GraphQLクエリファイルを追加/更新: `src/graphql/queries/*.graphql`, `src/graphql/mutations/*.graphql`
4. 型とhooksを生成: `pnpm codegen`
5. フロントエンドで生成されたhooksを使用

### 新しいクエリ/ミューテーションの追加

1. `src/graphql/queries/` または `src/graphql/mutations/` に`.graphql`ファイルを作成
2. GraphQLクエリを記述
3. `pnpm codegen`を実行して型とhooksを生成
4. 生成されたhooksをコンポーネントで使用

### 例: 新しいクエリの追加

```graphql
# src/graphql/queries/library.graphql
query GetBooksByAuthor($author: String!) {
  library {
    books {
      id
      title
      author
    }
  }
}
```

```bash
pnpm codegen
```

```typescript
// コンポーネントで使用
import { useGetBooksByAuthorQuery } from "~/generated/graphql"

function AuthorBooks({ author }: { author: string }) {
  const { data, loading } = useGetBooksByAuthorQuery({ variables: { author } })
  // ...
}
```

## 📝 今後の拡張案

1. **GraphQL Playground/IDE の統合**
   - ブラウザベースのGraphQLエクスプローラー
2. **認証・認可**
   - JWT認証
   - GraphQLのフィールドレベル認可

3. **リアルタイム機能**
   - GraphQL Subscriptions
   - WebSocket統合

4. **より複雑なリレーション**
   - 複数のエンティティ間の関係
   - JOIN操作

5. **ページネーション**
   - カーソルベースのページネーション
   - Relay仕様準拠

6. **キャッシング**
   - クエリ結果のキャッシュ
   - DataLoader パターン

7. **バリデーション強化**
   - カスタムバリデータ
   - エラーメッセージの改善

## 🔍 デバッグ

### GraphQLクエリのデバッグ

Rust側でログを追加：

```rust
println!("Executing query: {}", query);
```

### データベースクエリの確認

SeaORMのログを有効化するには、`Cargo.toml`に追加：

```toml
[dependencies]
tracing-subscriber = "0.3"
```

そして`lib.rs`で初期化：

```rust
tracing_subscriber::fmt::init();
```

## 📚 参考リンク

- [Tauri Documentation](https://tauri.app/)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)
- [async-graphql Documentation](https://async-graphql.github.io/async-graphql/)
- [GraphQL Specification](https://spec.graphql.org/)
