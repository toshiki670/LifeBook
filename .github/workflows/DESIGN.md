# CI Workflow Design Document

## 概要

このドキュメントはLifeBookプロジェクトのCI/CDワークフローの設計を記述します。

## 目的

- mainブランチとの差分があるファイルのみに対してチェックを実行することでCI時間を短縮
- フロントエンドとバックエンドで独立したワークフローを実行し、並列化による高速化
- lint-staged設定（`package.json`）と整合性のある動作を保証
- 外部アクション依存を最小化しセキュリティを向上

## アーキテクチャ

### ワークフロー構成

プロジェクトは2つの独立したワークフローに分割されています：

1. **Frontend CI** (`.github/workflows/frontend.yml`)
2. **Rust CI** (`.github/workflows/rust.yml`)

この分割により、フロントエンドとバックエンドの変更が独立して並列実行され、全体的なCI時間が短縮されます。

---

## Frontend CI Workflow

### トリガー条件

```yaml
on:
  pull_request:
    types: [opened, synchronize, reopened]
    paths:
      - "src/**"
      - "package.json"
      - "pnpm-lock.yaml"
      - ".github/workflows/frontend.yml"
      - "biome.jsonc"
      - "tsconfig.json"
```

### ジョブ構成

#### 1. Biome (Lint & Format)

**目的**: コードの品質とスタイルをチェック

**動作フロー**:

```
1. リポジトリをチェックアウト (fetch-depth: 0)
2. mainブランチをフェッチ
3. 差分ファイルを取得
   - git diff --name-only origin/main...HEAD
   - 拡張子フィルタ: .ts, .tsx, .js, .jsx, .json, .jsonc, .md
4. 差分ファイルがある場合のみ:
   - pnpm セットアップ
   - Node.js セットアップ
   - 依存関係インストール
   - Biome format チェック実行
   - Biome lint チェック実行
```

**lint-staged対応**:

```json
"*.{ts,tsx,js,jsx,json,jsonc,md}": [
  "biome format --write --no-errors-on-unmatched",
  "biome check --write --no-errors-on-unmatched"
]
```

**コマンド**:

- `pnpm biome format --no-errors-on-unmatched <差分ファイル>`
- `pnpm biome check --no-errors-on-unmatched <差分ファイル>`

#### 2. TypeScript Type Check

**目的**: TypeScriptの型チェック

**動作フロー**:

```
1. リポジトリをチェックアウト (fetch-depth: 0)
2. mainブランチをフェッチ
3. src/配下の変更をチェック
   - git diff --name-only origin/main...HEAD | grep '^src/'
4. src/配下に変更がある場合のみ:
   - pnpm セットアップ
   - Node.js セットアップ
   - 依存関係インストール
   - TypeScript型チェック実行
```

**lint-staged対応**:

```json
"src/**/*": [
  "bash -c 'pnpm run typecheck'"
]
```

**コマンド**:

- `pnpm typecheck` (全体チェック)

**注意**: TypeScriptの型チェックは差分ファイルのみでは不十分なため、src/配下に変更がある場合は全体をチェックします。

---

## Rust CI Workflow

### トリガー条件

```yaml
on:
  pull_request:
    types: [opened, synchronize, reopened]
    paths:
      - "src-tauri/**"
      - "Cargo.toml"
      - "Cargo.lock"
      - ".github/workflows/rust.yml"
```

### 環境変数

```yaml
env:
  CARGO_TERM_COLOR: always
```

### ジョブ構成

#### 1. Format Check

**目的**: Rustコードのフォーマットチェック

**実行環境**:

- タイムアウト: 5分
- ツールチェーン: stable
- コンポーネント: rustfmt

**動作フロー**:

```
1. リポジトリをチェックアウト (fetch-depth: 0)
2. mainブランチをフェッチ
3. src-tauri/配下の変更をチェック
4. src-tauri/配下に変更がある場合のみ:
   - Rust toolchain (stable) セットアップ
   - フォーマットチェック実行
```

**lint-staged対応**:

```json
"src-tauri/**/*": [
  "bash -c 'cd src-tauri && cargo fmt --all'"
]
```

**コマンド**:

- `cargo fmt --all --check`

#### 2. Clippy Lint

**目的**: Rustコードの静的解析

**実行環境**:

- タイムアウト: 10分
- ツールチェーン: stable
- コンポーネント: clippy

**動作フロー**:

```
1. リポジトリをチェックアウト (fetch-depth: 0)
2. mainブランチをフェッチ
3. src-tauri/配下の変更をチェック
4. src-tauri/配下に変更がある場合のみ:
   - システム依存関係インストール (Tauri required)
   - Rust toolchain (stable) セットアップ
   - Clippy実行
```

**lint-staged対応**:

```json
"src-tauri/**/*": [
  "bash -c 'cd src-tauri && cargo clippy --fix --allow-dirty --allow-staged -- -D warnings'"
]
```

**コマンド**:

- `cargo clippy --all-targets --all-features -- -D warnings`

**システム依存関係**:

- libwebkit2gtk-4.1-dev
- build-essential
- curl, wget, file
- libssl-dev
- libayatana-appindicator3-dev
- librsvg2-dev

#### 3. Tests

**目的**: Rustのユニット/統合テスト実行

**実行環境**:

- タイムアウト: 10分
- ツールチェーン: stable

**動作フロー**:

```
1. リポジトリをチェックアウト (fetch-depth: 0)
2. mainブランチをフェッチ
3. src-tauri/配下の変更をチェック
4. src-tauri/配下に変更がある場合のみ:
   - システム依存関係インストール
   - Rust toolchain (stable) セットアップ
   - 全テスト実行
```

**コマンド**:

- `cargo test --workspace --verbose`

**注意**: src-tauri/配下に変更がある場合、依存関係の変更により他のクレートに影響がある可能性があるため、ワークスペース全体のテストを実行します。

---

## 差分チェック実装

### 技術選定

**採用**: Gitコマンド (`git diff`)

**不採用**: 外部アクション (tj-actions/changed-files)

**理由**:

- セキュリティ: 外部アクション依存を排除（CVE-2025-30066などのリスク回避）
- シンプル: Gitコマンドのみでシンプルな実装
- 信頼性: GitHub標準のGitツールに依存

### 差分取得方法

```bash
# mainブランチのフェッチ
git fetch origin main:main

# 差分ファイル一覧の取得
git diff --name-only origin/main...HEAD

# 拡張子フィルタリング (例: TypeScript関連)
git diff --name-only origin/main...HEAD | grep -E '\.(ts|tsx|js|jsx|json|jsonc|md)$'

# ディレクトリフィルタリング (例: src-tauri配下)
git diff --name-only origin/main...HEAD | grep -q '^src-tauri/'
```

### 条件付き実行

**パターン1: ファイルリストを使用**

```yaml
- name: Get changed files
  id: changed-files
  run: |
    BIOME_FILES=$(git diff --name-only origin/main...HEAD | grep -E '\.(ts|tsx|js|jsx|json|jsonc|md)$' | tr '\n' ' ' || echo "")
    echo "biome_files=$BIOME_FILES" >> $GITHUB_OUTPUT
    echo "has_biome_files=$([ -n "$BIOME_FILES" ] && echo 'true' || echo 'false')" >> $GITHUB_OUTPUT

- name: Run check
  if: steps.changed-files.outputs.has_biome_files == 'true'
  run: command ${{ steps.changed-files.outputs.biome_files }}
```

**パターン2: ディレクトリ変更チェック**

```yaml
- name: Check directory changes
  id: dir-check
  run: |
    if git diff --name-only origin/main...HEAD | grep -q '^src-tauri/'; then
      echo "changed=true" >> $GITHUB_OUTPUT
    else
      echo "changed=false" >> $GITHUB_OUTPUT
    fi

- name: Run check
  if: steps.dir-check.outputs.changed == 'true'
  run: command
```

---

## lint-staged との整合性

### package.json の lint-staged 設定

```json
{
  "lint-staged": {
    "*.{ts,tsx,js,jsx,json,jsonc,md}": [
      "biome format --write --no-errors-on-unmatched",
      "biome check --write --no-errors-on-unmatched"
    ],
    "src/**/*": ["bash -c 'pnpm run typecheck'"],
    "src-tauri/**/*": [
      "bash -c 'cd src-tauri && cargo clippy --fix --allow-dirty --allow-staged -- -D warnings'",
      "bash -c 'cd src-tauri && cargo fmt --all'"
    ]
  }
}
```

### CI との対応関係

| lint-staged パターン              | CI ワークフロー          | チェック内容                   |
| --------------------------------- | ------------------------ | ------------------------------ |
| `*.{ts,tsx,js,jsx,json,jsonc,md}` | Frontend CI / Biome      | 差分ファイルのみ format & lint |
| `src/**/*`                        | Frontend CI / TypeScript | src/変更時に全体チェック       |
| `src-tauri/**/*`                  | Rust CI / Format         | src-tauri/変更時に全体チェック |
| `src-tauri/**/*`                  | Rust CI / Clippy         | src-tauri/変更時に全体チェック |
| -                                 | Rust CI / Tests          | src-tauri/変更時に全テスト実行 |

### Pre-commit と CI の違い

| 項目               | Pre-commit (lint-staged)  | CI Workflow          |
| ------------------ | ------------------------- | -------------------- |
| **実行タイミング** | ローカルコミット前        | PR作成/更新時        |
| **チェック範囲**   | ステージングファイル      | mainとの差分ファイル |
| **自動修正**       | あり (`--write`, `--fix`) | なし (チェックのみ)  |
| **テスト**         | なし                      | あり (Rust CI)       |
| **目的**           | 品質維持 + 自動整形       | 品質検証             |

---

## パフォーマンス最適化

### タイムアウト設定

| ジョブ                | タイムアウト | 理由                   |
| --------------------- | ------------ | ---------------------- |
| Frontend / Biome      | デフォルト   | 高速 (数秒)            |
| Frontend / TypeScript | デフォルト   | 高速 (数秒〜数十秒)    |
| Rust / Format         | 5分          | 高速だが余裕を持たせる |
| Rust / Clippy         | 10分         | ビルド時間を考慮       |
| Rust / Tests          | 10分         | テスト実行時間を考慮   |

### キャッシュ戦略

**Node.js依存関係**:

```yaml
- uses: actions/setup-node@v4
  with:
    cache: "pnpm"
```

**Rustコンパイル成果物**:

```yaml
- uses: actions-rust-lang/setup-rust-toolchain@v1
  with:
    cache-workspaces: src-tauri
```

### 並列実行

- Frontend CI と Rust CI は完全に独立して並列実行
- 各ワークフロー内の複数ジョブも並列実行
- 差分チェックにより不要なジョブはスキップ

---

## トラブルシューティング

### 問題: ワークフローがスキップされない

**原因**: `fetch-depth: 0` が設定されていない、または mainブランチのフェッチが失敗

**解決策**:

```yaml
- uses: actions/checkout@v4
  with:
    fetch-depth: 0 # 全履歴をフェッチ

- run: git fetch origin main:main # mainブランチを明示的にフェッチ
```

### 問題: Biome が差分ファイルを検出しない

**原因**: grep がマッチしない場合にエラーを返す

**解決策**:

```bash
# || echo "" でエラーを抑制
BIOME_FILES=$(git diff --name-only origin/main...HEAD | grep -E '\.(ts|tsx|js|jsx)$' | tr '\n' ' ' || echo "")
```

### 問題: Rust CI でシステム依存関係エラー

**原因**: Tauri に必要なシステムライブラリが不足

**解決策**: 必要なパッケージをすべてインストール

```yaml
- run: |
    sudo apt-get update
    sudo apt-get install -y libwebkit2gtk-4.1-dev \
      build-essential curl wget file \
      libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

---

## 将来の拡張

### リリース用CI (予定)

開発用CIとは別に、リリース用の厳格なCIを作成予定：

- **トリガー**: mainブランチへのマージ、リリースタグ
- **特徴**:
  - 差分チェックなし（全体チェック）
  - より長いタイムアウト
  - より厳格なチェック（追加のlintルールなど）
  - ビルド成果物の生成
  - デプロイメント

### Frontend テスト

将来的にフロントエンドのユニットテスト/E2Eテストを追加予定：

- Vitest/Jest によるユニットテスト
- Playwright/Cypress によるE2Eテスト

---

## 設計原則

1. **セキュリティファースト**: 外部アクション依存を最小化
2. **高速化**: 差分チェックと並列実行
3. **一貫性**: lint-stagedとの整合性を保つ
4. **安定性**: Stableツールチェーンを使用
5. **保守性**: シンプルでわかりやすい実装

---

## 変更履歴

- 2025-10-20: 初版作成
  - ワークフロー分割（Frontend CI / Rust CI）
  - 差分ベースチェック実装
  - Gitコマンドによる差分取得
  - Stableツールチェーン採用
  - タイムアウト調整
