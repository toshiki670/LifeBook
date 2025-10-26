# Prevent Direct Commits to Main Branch with Husky

## 概要
既存のHusky設定を利用してmainブランチへの直接コミットとマージをブロックし、ユーザーにfeature branchを作成するよう促す。バージョン管理されるため、チーム全体で自動的に適用される。

## 現状の確認
- Huskyは既にインストール済み（`package.json`に`husky: ^9.1.7`）
- `prepare`スクリプトで`husky`が設定済み
- lint-stagedも設定済み
- GitHubのブランチ保護ルールでPR必須が設定済み

## 実装内容

### 1. Huskyのpre-commitフック
`.husky/pre-commit`ファイルに以下を実装：
- 現在のブランチ名をチェック（`git rev-parse --abbrev-ref HEAD`）
- mainブランチの場合はエラーメッセージを表示してコミットを中止
- 代替フローの提案を含む詳細なメッセージ
- 既存のlint-staged処理も実行

### 2. Huskyのpre-merge-commitフック
`.husky/pre-merge-commit`ファイルに以下を実装：
- 現在のブランチ名をチェック
- mainブランチの場合はエラーメッセージを表示してマージを中止
- 代替フローの提案を含む詳細なメッセージ

### エラーメッセージ内容

**pre-commit (コミット時):**
```
ERROR: Direct commits to 'main' branch are not allowed.

Please follow this workflow instead:
1. Create a new feature branch: git checkout -b feature/your-feature-name
2. Make your changes and commit to the feature branch
3. Push your feature branch: git push origin feature/your-feature-name
4. Create a Pull Request for review

If you need to switch to an existing branch:
  git checkout <branch-name>
```

**pre-merge-commit (マージ時):**
```
ERROR: Direct merges into 'main' branch are not allowed.

Please follow this workflow instead:
1. Create a Pull Request on GitHub
2. Have the PR reviewed and approved
3. Merge through GitHub's interface

Use 'git reset --hard HEAD' to cancel the merge if needed.
```

## 実装ファイル
- `.husky/pre-commit` - pre-commitフック（バージョン管理下）
- `.husky/pre-merge-commit` - pre-merge-commitフック（バージョン管理下）

## 防御レイヤー
1. **pre-commit**: mainブランチへの直接コミットをブロック
2. **pre-merge-commit**: mainブランチへの直接マージをブロック
3. **GitHubブランチ保護ルール**: PR必須を設定（サーバーサイドの防御）

## メリット
- バージョン管理されているため、`pnpm install`時に自動的に全開発者に適用
- 追加のセットアップ手順不要
- 既存のlint-stagedと統合可能
- クライアントサイドとサーバーサイドの2段階で保護

## 注意事項
- `pre-push`フックは検証が不十分なため実装していない
- AIが勝手にmainブランチをマージしないようにするための設定である
- 検証時は必ずクリーンな状態に戻す

