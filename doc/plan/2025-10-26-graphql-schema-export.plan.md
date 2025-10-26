# GraphQL Schema Export 機能の実装

## 概要

GraphQL スキーマを`schema.graphql`ファイルとしてエクスポートする機能を追加します。これによりフロントエンドのコード生成ツール（GraphQL Code Generator）が型定義を自動生成できるようになります。

## 実装内容

### 1. ブランチ作成

新しいブランチ `feature/graphql-schema-export` を作成します。

### 2. Rust 側の変更

#### `src-tauri/lifebook/Cargo.toml`

- `default-run = "lifebook"` を追加
- 新しいバイナリターゲット `export_schema` を追加

#### `src-tauri/lifebook/src/graphql_schema.rs`

- `build_schema_without_data()` 関数を追加
  - データベース接続なしでスキーマ定義のみを構築
  - スキーマエクスポート専用

#### `src-tauri/lifebook/src/lib.rs`

- モジュールの可視性を変更 (`mod` → `pub mod`)
  - `app_state`
  - `database`
  - `graphql_schema`
- バイナリから利用できるようにする

#### `src-tauri/lifebook/src/bin/export_schema.rs` (新規作成)

- スキーマを SDL 形式でエクスポートするバイナリ
- プロジェクトルートの `schema.graphql` に出力
- データベース接続不要

### 3. package.json の変更

新しい npm スクリプトを追加:

```json
"export-schema": "cd src-tauri && cargo run --bin export_schema"
```

### 4. .gitignore の変更

生成ファイルを Git 管理対象外にする:

```
schema.graphql
```

## 技術的な詳細

- スキーマエクスポートはビルド時やフロントエンド開発時に実行
- `build_schema_without_data()` はデータベース接続や AppState が不要なため、CI/CD 環境でも安全に実行可能
- 生成される `schema.graphql` は自動生成ファイルのため Git 管理対象外

## 実装結果

### 変更されたファイル

1. `src-tauri/lifebook/Cargo.toml`

   - `default-run` フィールド追加
   - `[[bin]]` ターゲット追加

2. `src-tauri/lifebook/src/lib.rs`

   - `pub mod` に変更してバイナリからアクセス可能に

3. `src-tauri/lifebook/src/graphql_schema.rs`

   - `build_schema_without_data()` 関数追加

4. `src-tauri/lifebook/src/bin/export_schema.rs` (新規)

   - スキーマを SDL 形式でエクスポートするバイナリ

5. `package.json`

   - `export-schema` スクリプト追加

6. `.gitignore`
   - `schema.graphql` を追加

### 生成されるファイル

- `schema.graphql` - GraphQL スキーマ定義（SDL 形式、126 行）

### 使用方法

```bash
# スキーマをエクスポート
pnpm export-schema

# 生成されたスキーマを確認
cat schema.graphql
```

## 参考

- [PR #39](https://github.com/toshiki670/LifeBook/pull/39) - GraphQL Code Generator の near-operation-file プリセット検証スパイク
