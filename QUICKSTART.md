# クイックスタートガイド 🚀

## 📋 必要なもの

- Node.js (v18以上)
- pnpm
- Rust (最新の安定版)

## 🏃 実行手順

### 1. 依存関係のインストール

```bash
pnpm install
```

### 2. 開発サーバーの起動

```bash
pnpm tauri dev
```

初回起動時は、Rustの依存関係のダウンロードとコンパイルに数分かかります。

### 3. アプリケーションの使用

1. アプリケーションが起動したら、ホーム画面から **「📚 Book Manager (GraphQL Demo) へ」** ボタンをクリック
2. 本管理ページで以下の操作が可能です：
   - 📖 本の追加（タイトル、著者、説明、出版年）
   - 📋 本のリスト表示
   - 🗑️ 本の削除

## 🔍 動作確認

### データベース接続の確認

本管理ページ上部に「DB Status: Connected」と表示されていれば、正常に動作しています。

### GraphQLクエリのテスト

開発者ツール（DevTools）のコンソールで以下を実行できます：

```javascript
import { getBooks } from "./lib/graphql";
const result = await getBooks();
console.log(result);
```

## 📚 次のステップ

- [GRAPHQL_GUIDE.md](./GRAPHQL_GUIDE.md) - GraphQL統合の詳細な説明
- [README.md](./README.md) - プロジェクト概要

## 🔄 マイグレーションについて

このプロジェクトは、[SeaORM公式ドキュメント](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/)に従って、`sea-orm-migration`を使用しています。

- マイグレーションファイル: `src-tauri/src/migration.rs`（単一ファイル）
- アプリケーション起動時に自動実行されます
- データベーススキーマはマイグレーションで管理されています

**注**: このプロジェクトでは`mod.rs`を使わない方針を採用しています。詳細は[CODING_GUIDELINES.md](./CODING_GUIDELINES.md)を参照してください。

新しいマイグレーションを追加する場合：

1. `src-tauri/src/migration.rs`を開く
2. ファイル末尾に新しいマイグレーションモジュールを追加
3. `Migrator::migrations()`に新しいマイグレーションを登録
4. アプリを再起動すると自動的に適用されます

## ❗ トラブルシューティング

### エラー: "Failed to initialize database"

データベースファイルの権限を確認してください：

```bash
ls -la src-tauri/lifebook.db
```

### エラー: "Failed to run migrations"

マイグレーションに問題がある場合、データベースファイルを削除して再起動してください：

```bash
rm src-tauri/lifebook.db
pnpm tauri dev
```

アプリケーション起動時にマイグレーションが自動実行され、テーブルが再作成されます。

### Rustのコンパイルエラー

依存関係を更新してください：

```bash
cd src-tauri
cargo clean
cargo build
cd ..
pnpm tauri dev
```

## 🎯 サンプルデータ

アプリケーション起動後、以下のサンプルデータを追加してみてください：

1. **The Rust Programming Language**
   - 著者: Steve Klabnik, Carol Nichols
   - 出版年: 2018
   - 説明: Official guide to learning Rust

2. **Designing Data-Intensive Applications**
   - 著者: Martin Kleppmann
   - 出版年: 2017
   - 説明: The big ideas behind reliable, scalable systems

3. **Clean Code**
   - 著者: Robert C. Martin
   - 出版年: 2008
   - 説明: A handbook of agile software craftsmanship

これでTauri + SeaORM + GraphQLの統合を体験できます！🎉
