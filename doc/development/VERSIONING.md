# バージョニングガイド

このドキュメントでは、LifeBookプロジェクトのバージョン管理戦略とルールを定義します。

## 基本方針

- **セマンティックバージョニング (SemVer)** を採用
- **モノレポ統一バージョン戦略**: フロントエンド・バックエンド全体で単一のバージョン番号を使用
- `package.json` を **Source of Truth** とし、ビルド時に自動同期

## バージョン番号の形式

`MAJOR.MINOR.PATCH` (例: `1.2.3`)

## バージョニングルール

### 0.x.y（プレリリース・開発段階）

開発初期段階では、安定性よりも迅速な開発を優先します。

- **x（マイナー）**: 破壊的変更を含む開発を自由に実施
  - データベーススキーマの変更
  - GraphQL APIの破壊的変更
  - 大規模なリファクタリング
  - 新機能の追加

- **y（パッチ）**: バグフィックスのみ
  - 既存機能の不具合修正
  - パフォーマンス改善（互換性を保つもの）

**例:**

- `0.1.0` → `0.2.0`: 破壊的変更や新機能追加
- `0.2.0` → `0.2.1`: バグフィックス
- `0.2.1` → `0.3.0`: さらなる破壊的変更

### 1.x.y以降（安定版）

バージョン1.0.0以降は、ユーザーへの影響を最小限に抑え、安定性を重視します。

- **MAJOR（メジャー）**: データベースMigrationが発生する変更
  - データベーススキーマの変更（テーブル追加、カラム変更など）
  - ユーザーデータの移行が必要な変更
  - 既存のデータベース構造に影響を与える変更

- **MINOR（マイナー）**: Migration不要だがAPI変更を含む機能追加
  - GraphQL Schemaの変更（フィールド追加、型変更など）
  - 新機能の追加（データ構造に影響しない）
  - 後方互換性のあるAPI拡張

- **PATCH（パッチ）**: バグフィックスのみ
  - 既存機能の不具合修正
  - パフォーマンス改善
  - セキュリティ修正
  - 機能追加なし、Migration なし

**例:**

- `1.0.0` → `2.0.0`: データベースMigrationが必要
- `1.0.0` → `1.1.0`: GraphQL APIに新しいフィールドを追加
- `1.0.0` → `1.0.1`: バグフィックス

## バージョン管理の仕組み

### ファイル構成

```
LifeBook/
├── package.json              # Source of Truth（バージョンの起点）
├── src-tauri/
│   ├── Cargo.toml           # Workspace設定（version定義）
│   ├── lifebook/
│   │   ├── Cargo.toml       # version.workspace = true
│   │   └── tauri.conf.json  # 自動同期される
│   └── contexts/
│       ├── library/Cargo.toml    # version.workspace = true
│       ├── settings/Cargo.toml   # version.workspace = true
│       └── shared/Cargo.toml     # version.workspace = true
└── scripts/
    └── sync-version.js      # 同期スクリプト
```

### バージョン同期フロー

1. **手動更新**: `package.json` の `version` フィールドを更新
2. **自動同期**: `pnpm dev` または `pnpm build` 時に自動実行
   - `scripts/sync-version.js` が実行される
   - `package.json` → `tauri.conf.json` へバージョンをコピー
3. **Workspace同期**: Rust crateは `src-tauri/Cargo.toml` の workspace設定を参照

### バージョン更新ワークフロー

#### 1. バージョンの決定

変更内容に基づいてバージョンを決定：

```bash
# 現在のバージョンを確認
grep '"version"' package.json

# 変更内容を確認
git log --oneline HEAD~10..HEAD
```

#### 2. バージョンの更新

```bash
# package.jsonを更新（手動またはnpm version）
npm version patch    # 1.0.0 → 1.0.1
npm version minor    # 1.0.0 → 1.1.0
npm version major    # 1.0.0 → 2.0.0

# または手動で package.json を編集
```

#### 3. 同期の確認

```bash
# 開発サーバー起動時に自動同期
pnpm dev

# またはビルド時に自動同期
pnpm build

# 手動で同期を実行
pnpm sync-version
```

#### 4. 変更のコミット

```bash
git add package.json src-tauri/lifebook/tauri.conf.json
git commit -m "chore: bump version to x.y.z"
git tag -a vx.y.z -m "Release version x.y.z"
git push origin main --tags
```

## Cargo Workspace設定

Rust側では、Cargo Workspaceの機能を使ってバージョンを統一しています。

**src-tauri/Cargo.toml:**

```toml
[workspace.package]
edition = "2024"
version = "0.1.0"
```

**各crateのCargo.toml:**

```toml
[package]
name = "library"
version.workspace = true
edition.workspace = true
```

## バージョニングツール候補（将来的な導入検討）

現在は手動運用ですが、将来的に以下のツールの導入を検討できます：

### Rust向け

#### cargo-release

- Cargo workspace完全対応
- Changelog自動生成
- Git tag自動作成
- 依存関係の自動更新

```bash
cargo install cargo-release
cargo release patch --execute
```

#### cargo-workspaces

- モノレポ特化ツール
- 複数crateの一括バージョン更新
- 選択的リリース対応

```bash
cargo install cargo-workspaces
cargo workspaces version patch
```

### JavaScript/全体向け

#### changesets

- モノレポ向け設計
- チームでの変更管理に最適
- プルリクエストベースのワークフロー

```bash
pnpm add -D @changesets/cli
pnpm changeset init
```

#### standard-version

- Conventional Commits対応
- Changelog自動生成
- シンプルな操作性

```bash
pnpm add -D standard-version
pnpm standard-version
```

#### semantic-release

- CI/CD統合
- 完全自動リリース
- GitHub Releases連携

```bash
pnpm add -D semantic-release
```

### 推奨アプローチ

1. **現在**: 手動運用でルールを確立し、チームで慣れる
2. **次のステップ**: `standard-version` または `changesets` を導入してChangelog生成を自動化
3. **将来**: CI/CD環境が整ったら `semantic-release` で完全自動化を検討

## よくある質問

### Q: データベースMigrationとは？

A: データベーススキーマの変更を指します。例えば：

- 新しいテーブルの追加
- 既存テーブルへのカラム追加・削除・変更
- インデックスの追加・削除
- データ型の変更

### Q: GraphQL Schema変更はマイナーバージョン？

A: はい。1.x.y以降では、GraphQL Schemaの変更（フィールド追加、型変更など）はマイナーバージョンアップです。ただし、データベースMigrationを伴う場合はメジャーバージョンアップになります。

### Q: 0.x.y から 1.0.0 への移行タイミングは？

A: 以下の条件を満たした時：

- コア機能が完成し安定している
- APIが固まり、破壊的変更を避けたい状態
- 実際のユーザーに提供する準備が整っている

### Q: Hotfixの扱いは？

A: パッチバージョンを使用します。例えば本番環境で重大なバグが見つかった場合：

```
1.2.3 → 1.2.4 (hotfix)
```

## 参考リンク

- [Semantic Versioning 2.0.0](https://semver.org/)
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Tauri Configuration](https://tauri.app/v1/api/config/)
