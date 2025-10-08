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
├── lib.rs              # メインエントリーポイント、Tauriコマンド定義
├── database.rs         # データベース接続管理
├── migration.rs        # DBマイグレーション (全マイグレーションを含む)
├── entities.rs         # SeaORMエンティティ (全エンティティを含む)
└── graphql/            # GraphQL実装
    ├── mod.rs
    ├── schema.rs       # GraphQLスキーマ構築
    └── resolvers.rs    # Query/Mutationリゾルバー
```

**注**: このプロジェクトでは`mod.rs`を使わない方針を採用しています。詳細は[CODING_GUIDELINES.md](./CODING_GUIDELINES.md)を参照してください。

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

### 3. SeaORMエンティティ (`entities.rs`)

**重要**: このプロジェクトでは`mod.rs`を使わず、単一の`entities.rs`ファイルに全エンティティをまとめています。

```rust
use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub mod book {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
    #[sea_orm(table_name = "books")]
    #[graphql(concrete(name = "Book", params()))]
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
}
```

GraphQLの`SimpleObject`とSeaORMの`DeriveEntityModel`を組み合わせることで、
単一のモデルでORMとGraphQLの両方に対応できます。

新しいエンティティを追加する場合は、同じファイル内に`pub mod`ブロックを追加します。

### 4. GraphQLスキーマ (`graphql/resolvers.rs`)

#### Query

- `books`: すべての本を取得
- `book(id: Int!)`: IDで本を取得

#### Mutation

- `createBook(...)`: 新しい本を作成
- `updateBook(...)`: 本を更新
- `deleteBook(id: Int!)`: 本を削除

### 5. Tauriコマンド (`lib.rs`)

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

### 6. フロントエンドクライアント (`lib/graphql.ts`)

```typescript
export async function executeGraphQL<T = any>(
  query: string
): Promise<GraphQLResponse<T>> {
  const result = await invoke<string>("execute_graphql", { query });
  return JSON.parse(result);
}
```

便利なヘルパー関数も提供：

- `getBooks()`: すべての本を取得
- `getBook(id)`: IDで本を取得
- `createBook(book)`: 本を作成
- `updateBook(id, updates)`: 本を更新
- `deleteBook(id)`: 本を削除

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

GraphQLレスポンスには`data`と`errors`の両方が含まれる可能性があるため、
フロントエンド側で適切に処理：

```typescript
const response = await getBooks();
if (response.errors) {
  // エラー処理
} else if (response.data) {
  // 成功時の処理
}
```

### 4. 型安全性

- Rust側: SeaORMとasync-graphqlの型システムを活用
- TypeScript側: GraphQLレスポンスの型を定義

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
