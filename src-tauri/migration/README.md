# LifeBook Database Migration

このディレクトリには、LifeBookアプリケーションのデータベースマイグレーションが含まれています。

## SeaORM CLI のセットアップ

グローバルに SeaORM CLI をインストールしてください：

```bash
cargo install sea-orm-cli
```

## マイグレーションの管理

### 新しいマイグレーションの作成

```bash
cd src-tauri
sea-orm-cli migrate generate <migration_name>
```

例：
```bash
cd src-tauri
sea-orm-cli migrate generate add_isbn_to_books
```

これにより、`migration/src/` ディレクトリに新しいマイグレーションファイルが作成されます。
作成後、`migration/src/lib.rs` の `migrations()` 関数に新しいマイグレーションを追加してください。

### マイグレーションの適用

```bash
cd src-tauri
sea-orm-cli migrate up
```

または、プロジェクトルートから：
```bash
cd src-tauri && sea-orm-cli migrate up
```

### マイグレーションのロールバック

```bash
cd src-tauri
sea-orm-cli migrate down
```

特定の数のマイグレーションをロールバック：
```bash
cd src-tauri
sea-orm-cli migrate down -n 2
```

### マイグレーションステータスの確認

```bash
cd src-tauri
sea-orm-cli migrate status
```

### マイグレーションのリフレッシュ（すべてロールバック→再適用）

```bash
cd src-tauri
sea-orm-cli migrate fresh
```

**注意**: これはすべてのデータを削除します！

## データベース接続

マイグレーションは、環境変数 `DATABASE_URL` で指定されたデータベースに対して実行されます。
デフォルトでは `sqlite://lifebook.db?mode=rwc` が使用されます。

環境変数で上書きする場合：
```bash
export DATABASE_URL="sqlite://path/to/database.db?mode=rwc"
sea-orm-cli migrate up
```

## 開発ワークフロー

1. **マイグレーションの作成**: `sea-orm-cli migrate generate <name>`
2. **マイグレーションの編集**: 生成されたファイルを編集して、`up()` と `down()` を実装
3. **マイグレーションの登録**: `src/lib.rs` の `migrations()` に追加
4. **マイグレーションの適用**: `sea-orm-cli migrate up`
5. **動作確認**: アプリケーションを起動してデータベーススキーマを確認

## 既存のマイグレーション

- `m20250108_000001_create_book_table.rs` - 書籍テーブルの作成

