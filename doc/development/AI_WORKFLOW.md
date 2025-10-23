# AI開発ワークフロー

このドキュメントは、AI（Cursor）を使用した開発フローを定義します。AIアシスタントはこのワークフローに従って作業を進めてください。

## Planモード時のワークフロー

### 1. ブランチの作成

**重要**: Planモードで実装を開始する際は、必ず新しいブランチを切ってから作業を開始すること。

```bash
git checkout -b <branch-type>/<feature-name>
```

#### ブランチ命名規則

- `feat/` - 新機能の追加
- `fix/` - バグ修正
- `docs/` - ドキュメントの変更
- `refactor/` - リファクタリング
- `test/` - テストの追加・修正
- `chore/` - ビルドプロセスやツールの変更

例:
```bash
git checkout -b docs/organize-documentation
git checkout -b feat/add-user-authentication
git checkout -b fix/resolve-login-issue
```

### 2. Planファイルの作成と管理

#### Planファイルの配置場所

すべてのPlanファイルは `doc/plan/` ディレクトリに配置してください。

#### Planファイルの命名規則

```
YYYY-MM-DD-<feature-or-task-name>.plan.md
```

例:
- `2025-10-23-organize-documentation.plan.md`
- `2025-10-24-add-user-authentication.plan.md`

#### Planファイルの必須項目

Planファイルには以下の情報を必ず含めてください：

1. **概要**: タスクの目的と背景
2. **実装手順**: 具体的な手順
3. **PRリンク**: 作成されたPull RequestのURL

**テンプレート**:

```markdown
# [タスク名]

## 概要

[タスクの目的と背景を記述]

## 実装手順

1. [手順1]
2. [手順2]
...

## Pull Request

- PR: https://github.com/[org]/[repo]/pull/[number]
- ブランチ: `[branch-name]`
- ステータス: [Open/Merged/Closed]

## 完了条件

- [ ] [条件1]
- [ ] [条件2]
```

### 3. PR作成時の注意事項

#### PRの作成

実装完了後、必ずPRを作成してください：

```bash
git push -u origin <branch-name>
gh pr create --title "<title>" --body "<description>"
```

#### PRのタイトル規則

Conventional Commitsに従ってください：

- `feat:` - 新機能
- `fix:` - バグ修正
- `docs:` - ドキュメントのみの変更
- `style:` - コードの意味に影響を与えない変更（空白、フォーマット等）
- `refactor:` - バグ修正や機能追加を行わないコード変更
- `perf:` - パフォーマンス改善
- `test:` - テストの追加や修正
- `chore:` - ビルドプロセスやツールの変更

例:
```
feat: add user authentication system
docs: organize documentation into doc directory structure
fix: resolve login session timeout issue
```

#### PlanファイルへのPRリンク追加

PR作成後、必ずPlanファイルにPRのリンクを追加してコミット・プッシュしてください：

```markdown
## Pull Request

- PR: https://github.com/[org]/[repo]/pull/[number]
- ブランチ: `[branch-name]`
- ステータス: Open
```

### 4. 完了フロー

1. ✅ ブランチを作成
2. ✅ Planファイルを作成（`doc/plan/YYYY-MM-DD-task-name.plan.md`）
3. ✅ 実装を完了
4. ✅ PRを作成
5. ✅ PlanファイルにPRリンクを追加してコミット・プッシュ
6. ✅ レビュー待ち・マージ

## コミットメッセージ規則

Conventional Commitsに従ってください：

```
<type>(<scope>): <subject>

<body>

<footer>
```

### 例

```
feat(auth): add JWT token authentication

Implement JWT-based authentication system with refresh tokens.
- Add JWT middleware
- Create token generation utilities
- Implement refresh token rotation

Closes #123
```

## チェックリスト

Planモード実装前に確認してください：

- [ ] ブランチを切っているか？
- [ ] ブランチ名は規則に従っているか？
- [ ] Planファイルを作成したか？
- [ ] Planファイル名は規則に従っているか？（YYYY-MM-DD-feature-name.plan.md）

PR作成前に確認してください：

- [ ] コミットメッセージは規則に従っているか？
- [ ] PRタイトルは規則に従っているか？
- [ ] PRの説明は十分か？
- [ ] Planファイルを更新したか？

PR作成後に確認してください：

- [ ] PlanファイルにPRリンクを追加したか？
- [ ] Planファイルの変更をコミット・プッシュしたか？

