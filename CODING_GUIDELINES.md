# コーディングガイドライン

このプロジェクトの開発における重要な方針とルールをまとめています。

## 🏗️ アーキテクチャ: ドメイン駆動設計（DDD）

このプロジェクトは**フルDDD（ドメイン駆動設計）**を採用しています。

### レイヤー構造

```
┌─────────────────────────────────────┐
│  Presentation Layer                 │  ← GraphQLリゾルバー (presentation/)
│  (GraphQL API)                      │     クライアントとの通信
├─────────────────────────────────────┤
│  Application Layer                  │  ← ユースケース、サービス (application/)
│  (Use Cases, Services, DTOs)        │     アプリケーションロジック
├─────────────────────────────────────┤
│  Domain Layer                       │  ← ビジネスロジック (domain/)
│  (Entities, Repository Traits)      │     ドメインモデル、ビジネスルール
├─────────────────────────────────────┤
│  Infrastructure Layer               │  ← データ永続化 (infrastructure/)
│  (Repository Impl, SeaORM, DB)      │     リポジトリ実装、DB操作
└─────────────────────────────────────┘
```

### ディレクトリ構造

```
src-tauri/src/
├── modules/                           # ドメインモジュール
│   ├── library/                       # 図書管理の境界づけられたコンテキスト
│   │   ├── domain/                    # Domain Layer
│   │   │   ├── entities/
│   │   │   │   └── book.rs            # ドメインエンティティ
│   │   │   └── repositories/
│   │   │       └── book.rs            # リポジトリtrait
│   │   ├── application/               # Application Layer
│   │   │   ├── dto/
│   │   │   │   └── book.rs            # データ転送オブジェクト
│   │   │   └── services/
│   │   │       └── book.rs            # アプリケーションサービス
│   │   └── infrastructure/            # Infrastructure Layer (実装部分)
│   │       └── repositories/
│   │           └── book.rs            # リポジトリ実装
│   └── shared/                        # 全コンテキスト共通
│       ├── domain/
│       │   └── errors.rs              # ドメインエラー
│       └── application/
│           └── errors.rs              # アプリケーションエラー
├── infrastructure/                    # 技術的詳細（コンテキスト外）
│   └── models/                        # SeaORM Models（全コンテキスト共有）
│       └── book.rs
├── presentation/                      # Presentation Layer
│   ├── schema.rs                      # GraphQLスキーマ構築
│   └── library/
│       ├── queries/
│       │   └── book.rs                # GraphQL Query
│       └── mutations/
│           └── book.rs                # GraphQL Mutation
├── app_state.rs                       # 依存性注入コンテナ
├── database.rs                        # データベース接続
├── migration.rs                       # マイグレーション
├── lib.rs                             # エントリーポイント
└── main.rs                            # バイナリエントリー
```

### 依存関係ルール

- **上位レイヤーは下位レイヤーに依存できる**が、**下位レイヤーは上位レイヤーに依存してはならない**
- **Domain層は他のどの層にも依存しない**（純粋なビジネスロジック）
- Infrastructure層はDomain層のインターフェース（trait）を実装する

### 各レイヤーの責務

#### 1. Domain Layer (`modules/{context}/domain/`)

**役割**: ビジネスロジックの中核

**ファイル構成**:

- `entities/book.rs` - ビジネスルールを持つドメインモデル
- `repositories/book.rs` - データ永続化のインターフェース定義

**共通エラー**: `modules/shared/domain/errors.rs`

**ルール**:

- 他のレイヤーに依存しない
- SeaORMなどの技術的詳細を含まない
- ビジネスルールのバリデーションはここで実装

**例**:

```rust
// modules/library/domain/entities/book.rs
use crate::modules::shared::domain::errors::DomainError;

pub struct Book {
    id: Option<i32>,
    title: String,
    // ...
}

impl Book {
    pub fn new(title: String, ...) -> Result<Self, DomainError> {
        // ビジネスルールのバリデーション
        if title.trim().is_empty() {
            return Err(DomainError::ValidationError(...));
        }
        Ok(Self { ... })
    }
}
```

#### 2. Application Layer (`modules/{context}/application/`)

**役割**: ユースケースの実装

**ファイル構成**:

- `services/book.rs` - ドメインを組み合わせてアプリケーション機能を提供
- `dto/book.rs` - プレゼンテーション層とのデータ転送オブジェクト

**共通エラー**: `modules/shared/application/errors.rs`

**ルール**:

- Domain層のリポジトリtraitを使用
- トランザクション境界を定義
- DTOを使って上位層にデータを渡す

**例**:

```rust
// modules/library/application/services/book.rs
use crate::modules::library::domain::{
    entities::book::Book,
    repositories::book::BookRepository,
};
use crate::modules::shared::application::errors::ApplicationError;

pub struct BookService {
    repository: Arc<dyn BookRepository>,
}

impl BookService {
    pub async fn create_book(...) -> Result<BookDto, ApplicationError> {
        let book = Book::new(...)?;  // ドメイン層
        let saved = self.repository.save(book).await?;
        Ok(BookDto::from(saved))
    }
}
```

#### 3. Infrastructure Layer

Infrastructure層は2つの場所に分かれています：

**A. コンテキスト内** (`modules/{context}/infrastructure/`)

**役割**: そのコンテキストのRepository実装

**ファイル構成**:

- `repositories/book.rs` - Domain層のリポジトリtraitを実装

**ルール**:

- ドメインモデル ↔ DBモデルの変換を担当
- 自コンテキストのDomain層と共有`infrastructure/models/`に依存

**例**:

```rust
// modules/library/infrastructure/repositories/book.rs
use crate::infrastructure::models::book;  // 共有Modelを参照
use crate::modules::library::domain::{
    entities::book::Book,
    repositories::book::BookRepository,
};

pub struct BookRepositoryImpl {
    db: DatabaseConnection,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn save(&self, book: Book) -> Result<Book, DomainError> {
        // ドメインモデル → SeaORMモデル変換
        let active_model = Self::domain_to_active_model(&book);
        let result = active_model.insert(&self.db).await?;
        // SeaORMモデル → ドメインモデル変換
        Ok(Self::db_to_domain(result))
    }
}
```

**B. コンテキスト外** (`infrastructure/`)

**役割**: SeaORM Models（DBスキーマ）

**ファイル構成**:

- `models/book.rs` - SeaORMモデル（全コンテキスト共有）

**理由**: SeaORMのRelation定義では、同じディレクトリ内の方が参照が容易

#### 4. Presentation Layer (`presentation/`)

**役割**: クライアントとの通信

**ファイル構成**:

- `schema.rs` - GraphQLスキーマ構築
- `library/queries/book.rs` - GraphQL Query（読み取り操作）
- `library/mutations/book.rs` - GraphQL Mutation（書き込み操作）

**ルール**:

- ビジネスロジックを含まない
- Application層のサービスに処理を委譲
- GraphQLエラーハンドリング

**例**:

```rust
// presentation/library/mutations/book.rs
use crate::app_state::AppState;
use crate::modules::library::application::dto::book::BookDto;

#[Object]
impl BookMutation {
    async fn create_book(&self, ctx: &Context<'_>, title: String, ...) -> Result<BookDto> {
        let app_state = ctx.data::<AppState>()?;
        app_state.book_service
            .create_book(title, ...)
            .await
            .map_err(|e| Error::new(e.to_string()))
    }
}
```

### DDDアーキテクチャのメリット

1. **テスタビリティ**: ドメインロジックが純粋なRustコードで、モック可能
2. **保守性**: 各レイヤーの責務が明確で変更の影響範囲が限定的
3. **拡張性**: REST APIやgRPC追加時もドメイン層は再利用可能
4. **ビジネスロジックの可視性**: `domain.rs`を見ればビジネスルールが理解できる

### 新機能の追加手順

例: `library`コンテキストに新しいエンティティ（Author）を追加する場合

1. **Domain層**:
   - `modules/library/domain/entities/author.rs` にエンティティを追加
   - `modules/library/domain/repositories/author.rs` にリポジトリtraitを追加
2. **Infrastructure層**:
   - `infrastructure/models/author.rs` にSeaORMモデルを追加（コンテキスト外）
   - `modules/library/infrastructure/repositories/author.rs` にリポジトリ実装を追加（コンテキスト内）
3. **Application層**:
   - `modules/library/application/services/author.rs` にサービスを追加
   - `modules/library/application/dto/author.rs` にDTOを追加
4. **Presentation層**:
   - `presentation/library/queries/author.rs` にクエリを追加
   - `presentation/library/mutations/author.rs` にミューテーションを追加
5. **依存性注入**:
   - `app_state.rs` でサービスを登録

詳細は `ARCHITECTURE.md` を参照してください。

## 📁 モジュール構造の方針

### ⚠️ 重要: `{folder}.rs` パターンを使用する

このプロジェクトでは、**`mod.rs` ではなく `{folder}.rs` パターン**を採用しています。

#### ❌ 避けるべき構造（古い方式）

```
src/
├── entities/
│   ├── mod.rs          # ❌ エディタで複数のmod.rsが開いて混乱する
│   ├── book.rs
│   └── user.rs
└── migration/
    ├── mod.rs          # ❌ どのmod.rsか分かりにくい
    └── m20250108_create_table.rs
```

#### ✅ 推奨する構造（新しい方式）

```
src/
├── entities/
│   ├── book.rs
│   └── user.rs
├── entities.rs         # ✅ モジュール宣言（mod.rsの代わり）
└── migration.rs        # ✅ 単一ファイルの場合はそのまま
```

### 理由

1. **エディタの使いやすさ**: 複数の`mod.rs`タブが開かれて混乱することがない
2. **明確さ**: ファイル名でモジュールが明確になる（`entities.rs`は`entities/`モジュール）
3. **新しい推奨方式**: Rust 2018エディション以降で推奨されるパターン
4. **保守性**: モジュール宣言がファイル名で判別できる

### 実装例

#### ✅ 正しい実装例 1: 複数ファイルに分割する場合

```
src/
├── modules/
│   ├── library/
│   │   ├── domain/
│   │   │   ├── entities/
│   │   │   │   └── book.rs        # Bookエンティティの実装
│   │   │   └── entities.rs        # モジュール宣言（mod.rsの代わり）
│   │   └── domain.rs               # モジュール宣言
│   └── library.rs                  # モジュール宣言
└── modules.rs                      # モジュール宣言
```

**entities.rs（モジュール宣言ファイル）**:
```rust
// modules/library/domain/entities.rs
pub mod book;
// 将来authorを追加する場合
// pub mod author;
```

**book.rs（実装ファイル）**:
```rust
// modules/library/domain/entities/book.rs
use crate::modules::shared::domain::errors::DomainError;

pub struct Book {
    id: Option<i32>,
    title: String,
    // ...
}

impl Book {
    pub fn new(title: String, ...) -> Result<Self, DomainError> {
        // 実装
    }
}
```

#### ✅ 正しい実装例 2: 単一ファイルの場合（migration.rs）

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

### 1. `mod.rs` パターン（古い方式）

```rust
// ❌ 避ける
src/
└── entities/
    ├── mod.rs      # エディタで複数のmod.rsが開いて混乱
    └── book.rs
```

### 2. 過度なモジュール分割

小規模なプロジェクトでは、関連するコードは同じファイルにまとめる方が管理しやすい。

## ✅ 推奨パターン

### 1. `{folder}.rs` パターン（新しい方式）

```rust
// ✅ 推奨
src/
├── entities/
│   └── book.rs
└── entities.rs    # モジュール宣言（mod.rsの代わり）
```

### 2. 単一ファイルでまとめる（小規模な場合）

ファイルが500行以下で、関連コードが少ない場合は単一ファイルにまとめる。

```rust
// ✅ 小規模なら単一ファイルでOK
src/
└── migration.rs   # すべてのマイグレーションを含む
```

### 3. ディレクトリ構造の例

```
src/
├── modules/
│   ├── library/
│   │   └── domain/
│   │       ├── entities/
│   │       │   └── book.rs
│   │       └── entities.rs  # モジュール宣言
│   └── library.rs            # モジュール宣言
└── modules.rs                # モジュール宣言
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
