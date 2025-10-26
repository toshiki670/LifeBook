# Plan Documents

このディレクトリには、Cursor Planモードで作成される計画ドキュメントを格納します。

## ファイル命名規則

Planファイルには必ず日付を付けてください。以下の形式を推奨します：

```
YYYY-MM-DD-<feature-or-task-name>.plan.md
```

日付は手動で入力せず、以下のコマンドを使用してください：

```bash
# 最新の日付を取得
date +%Y-%m-%d

# ファイル作成例
cat > doc/plan/$(date +%Y-%m-%d)-<feature-name>.plan.md << 'EOF'
# Your plan content here
EOF
```

### 例

- `2025-10-23-organize-documentation.plan.md`
- `2025-10-24-add-user-authentication.plan.md`
- `2025-11-01-refactor-database-layer.plan.md`

## 目的

- プロジェクトの計画段階で作成されたドキュメントを一元管理
- 時系列で計画の履歴を追跡可能にする
- 実装前の設計思想や意思決定の記録を保持
