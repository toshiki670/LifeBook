# コーディングガイドライン

このプロジェクトの開発における重要な方針とルールをまとめています。

## 📁 モジュール構造の方針

### ⚠️ 重要: `mod.rs` を使用しない

このプロジェクトでは、**`mod.rs` ファイルを作成しない**方針を採用しています。

#### ❌ 避けるべき構造

```
src/
├── entities/
│   ├── mod.rs          # ❌ 使わない
│   ├── book.rs
│   └── user.rs
└── migration/
    ├── mod.rs          # ❌ 使わない
    └── m20250108_create_table.rs
```

#### ✅ 推奨する構造

```
src/
├── entities.rs         # ✅ 単一ファイルにまとめる
└── migration.rs        # ✅ 単一ファイルにまとめる
```

### 理由

1. **シンプルさ**: ファイル数が少なくなり、プロジェクト構造が把握しやすい
2. **保守性**: モジュール宣言が散らばらず、一箇所で管理できる
3. **明確さ**: ディレクトリとモジュールの対応関係が明確になる

### 実装例

#### ✅ 正しい実装: `entities.rs`

```rust
// Book Entity
use async_graphql::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub mod book {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
    #[sea_orm(table_name = "books")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub title: String,
        // ...
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// 今後、他のエンティティを追加する場合もこのファイルに追記
pub mod user {
    use super::*;
    // User エンティティの実装
}
```

#### ✅ 正しい実装: `migration.rs`

```rust
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250108_000001_create_book_table::Migration),
            // 新しいマイグレーションはここに追加
        ]
    }
}

// Migration 1: Create Book Table
mod m20250108_000001_create_book_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            // マイグレーション実装
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            // ロールバック実装
        }
    }
}

// Migration 2: 新しいマイグレーションはこのファイルに追記
mod m20250109_000001_add_user_table {
    use super::*;
    // 実装...
}
```

## 🔄 マイグレーションの追加方法

新しいマイグレーションを追加する際の手順：

1. `src-tauri/src/migration.rs` を開く
2. ファイル末尾に新しいマイグレーションモジュールを追加
3. `Migrator::migrations()` に新しいマイグレーションを追加
4. アプリを再起動すると自動適用される

### 例: 新しいテーブルを追加

```rust
// migration.rs の末尾に追加

mod m20250109_000001_create_user_table {
    use super::*;

    #[derive(DeriveMigrationName)]
    pub struct Migration;

    #[async_trait::async_trait]
    impl MigrationTrait for Migration {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(User::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(User::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(User::Name).string().not_null())
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .drop_table(Table::drop().table(User::Table).to_owned())
                .await
        }
    }

    #[derive(DeriveIden)]
    enum User {
        Table,
        Id,
        Name,
    }
}
```

そして、`Migrator::migrations()`を更新：

```rust
fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
        Box::new(m20250108_000001_create_book_table::Migration),
        Box::new(m20250109_000001_create_user_table::Migration), // 追加
    ]
}
```

## 🏗️ プロジェクト構造

```
src-tauri/src/
├── lib.rs              # メインエントリーポイント
├── main.rs             # バイナリエントリーポイント
├── database.rs         # データベース接続管理
├── migration.rs        # 全マイグレーション (mod.rsを使わない)
├── entities.rs         # 全エンティティ (mod.rsを使わない)
└── graphql.rs または graphql/
    ├── schema.rs       # 必要に応じてサブモジュール化
    └── resolvers.rs
```

### モジュール宣言 (`lib.rs`)

```rust
mod database;
mod entities;     // entities.rs を参照
mod graphql;
mod migration;    // migration.rs を参照
```

## 📝 命名規則

### ファイル名

- **モジュールファイル**: `snake_case.rs` (例: `database.rs`, `migration.rs`)
- **マイグレーション**: `m{YYYYMMDD}_{連番}_{説明}.rs` の形式でモジュール名を付ける

### コード内

- **構造体/列挙型**: `PascalCase`
- **関数/変数**: `snake_case`
- **定数**: `SCREAMING_SNAKE_CASE`

## 🚫 避けるべきパターン

### 1. ディレクトリを作って `mod.rs` を配置する

```rust
// ❌ 避ける
src/
└── entities/
    ├── mod.rs
    └── book.rs
```

### 2. 過度なモジュール分割

小規模なプロジェクトでは、関連するコードは同じファイルにまとめる方が管理しやすい。

## ✅ 推奨パターン

### 1. 単一ファイルでモジュール管理

```rust
// ✅ 推奨
src/
└── entities.rs    # 内部でpub modを使って複数エンティティを定義
```

### 2. 必要に応じてサブモジュール化

ファイルが500行を超えるなど、明確に分割が必要な場合のみ、ディレクトリ構造を検討する。
その場合でも `mod.rs` は使わず、明示的なファイル名を使う。

```rust
// 大規模になった場合の例（現時点では不要）
src/
└── graphql/
    ├── query.rs      # QueryRootの実装
    ├── mutation.rs   # MutationRootの実装
    └── types.rs      # 共通型定義
```

そして `lib.rs` で：

```rust
mod graphql {
    pub mod query;
    pub mod mutation;
    pub mod types;
}
```

## ⚠️ トラブルシューティング

### マイグレーションエラー: "Migration file is missing"

**エラーメッセージ**:

```
Failed to run migrations: Custom("Migration file of version 'm20250108_000001_create_book_table' is missing, this migration has been applied but its file is missing")
```

**原因**:
マイグレーションファイルの構造を変更した後、データベースの`seaql_migrations`テーブルに古いマイグレーション履歴が残っているため。

**解決方法**:

#### オプション1: データベースを再作成（推奨・開発中の場合）

```bash
cd src-tauri
rm -f lifebook.db
pnpm tauri dev  # アプリを再起動すると、新しい構造でマイグレーションが実行される
```

#### オプション2: マイグレーションテーブルをクリア（本番データがある場合）

```bash
# SQLiteに接続
sqlite3 src-tauri/lifebook.db

# マイグレーション履歴をクリア
DELETE FROM seaql_migrations;

# 終了
.quit
```

その後、アプリを再起動すると、すべてのマイグレーションが再実行されます。

**注意**: オプション2の場合、既存のテーブル構造がマイグレーションと一致している必要があります。開発中は**オプション1を推奨**します。

### モジュール構造を変更した場合の手順

モジュール構造を変更した場合は、必ず以下の手順を実行してください：

1. データベースファイルを削除

   ```bash
   rm -f src-tauri/lifebook.db
   ```

2. アプリケーションを再起動

   ```bash
   pnpm tauri dev
   ```

3. マイグレーションが正常に実行されることを確認

## 🔗 参考リンク

- [The Rust Programming Language - Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust API Guidelines - Organization](https://rust-lang.github.io/api-guidelines/organization.html)
- [SeaORM Migration Guide](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/)

---

**最終更新**: 2025-01-08
