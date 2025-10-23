<!-- 99ae72e2-e4be-4fff-ad4a-d78d4d6abd19 8309bd56-1b0b-45fd-892b-0ecd506c70c8 -->
# ドキュメント構造の整理

## 概要

現在ルートディレクトリに散在しているドキュメントを、`doc`ディレクトリ配下に種別ごとに整理します。

## ディレクトリ構造

```
doc/
├── architecture/     # アーキテクチャ関連ドキュメント
│   └── ARCHITECTURE.md
├── development/      # 開発者向けドキュメント
│   ├── CODING_GUIDELINES.md
│   ├── GRAPHQL_GUIDE.md
│   ├── QUICKSTART.md
│   └── VERSIONING.md
└── plan/            # Planモードで作成される計画ドキュメント
    └── README.md    # このディレクトリの説明
```

ルートに残すファイル：

- README.md
- LICENSE

## 実装手順

1. `doc`ディレクトリと各サブディレクトリを作成
2. 既存ドキュメントを適切なディレクトリに移動

   - ARCHITECTURE.md → doc/architecture/
   - CODING_GUIDELINES.md, GRAPHQL_GUIDE.md, QUICKSTART.md, VERSIONING.md → doc/development/

3. doc/plan/README.md を作成（ディレクトリの目的を説明）
4. ルートのREADME.mdを更新（ドキュメント構造への参照を追加）

### To-dos

- [ ] docディレクトリと各サブディレクトリ（architecture, development, plan）を作成
- [ ] 既存ドキュメントを適切なディレクトリに移動
- [ ] doc/plan/README.mdを作成してディレクトリの目的を説明
- [ ] ルートのREADME.mdを更新してドキュメント構造への参照を追加

