# near-operation-file パターン検証結果

## 検証日

2025-10-25

## 概要

GraphQL Code Generatorの`near-operation-file`プリセットを使用して、クエリファイルと同じディレクトリに型定義を生成するパターンを検証しました。

## 実装結果

### 生成されたファイル構造

```
src/
├── features/
│   ├── books/
│   │   ├── queries.graphql
│   │   ├── queries.generated.ts (17行) ← 新規生成
│   │   ├── mutations.graphql
│   │   ├── mutations.generated.ts (34行) ← 新規生成
│   │   └── page.tsx
│   └── settings/
│       ├── queries.graphql
│       ├── queries.generated.ts (21行) ← 新規生成
│       ├── mutations.graphql
│       └── mutations.generated.ts (27行) ← 新規生成
└── generated/
    └── graphql.ts (基本型定義のみ、142行)
```

### 設定変更

#### codegen.yml

```yaml
generates:
  # 共通の型定義を生成
  src/generated/graphql.ts:
    plugins:
      - typescript
  # クエリファイルの近くに型定義を生成
  src/:
    preset: "near-operation-file"
    presetConfig:
      baseTypesPath: generated/graphql.ts
      extension: .generated.ts
    plugins:
      - typescript-operations
      - typed-document-node
```

#### インポートの変化

**変更前 (client プリセット):**

```typescript
import {
  BooksCreateDocument,
  BooksDeleteDocument,
  BooksGetAllDocument,
} from "~/generated/graphql";
```

**変更後 (near-operation-file プリセット):**

```typescript
import {
  BooksCreateDocument,
  BooksDeleteDocument,
} from "./mutations.generated";
import { BooksGetAllDocument } from "./queries.generated";
```

### 必要なパッケージ

- `@graphql-codegen/near-operation-file-preset` (新規追加)
- `@graphql-codegen/typed-document-node` (新規追加)
- `@graphql-codegen/typescript-operations` (既存)

## 評価結果

### ✅ メリット

1. **コロケーション**
   - GraphQLクエリと型定義が同じディレクトリに配置される
   - クエリの変更時に関連する型定義を見つけやすい

2. **インポートパスの簡潔さ**
   - 相対パス (`./queries.generated`) で簡潔
   - クエリとミューテーションを明確に分離できる

3. **ファイル構成の分かりやすさ**
   - 機能ごとにファイルがまとまっている
   - 各featureディレクトリが自己完結している

4. **型安全性**
   - DocumentNodeとして正しく生成される
   - `documentMode: "documentNode"` で動作確認済み

5. **ビルドサイズ**
   - 生成ファイルが小さく分割される（17〜34行）
   - Tree-shakingが効きやすい

### ⚠️ デメリット/注意点

1. **生成ファイル数の増加**
   - GraphQLファイルごとに`.generated.ts`ファイルが生成される
   - プロジェクトが大きくなると生成ファイルが多くなる

2. **インポートの複雑化**
   - 複数のクエリを使用する場合、複数のインポート文が必要
   - 単一ファイルからのインポートではなくなる

3. **基本型定義への依存**
   - `src/generated/graphql.ts`は引き続き必要
   - 完全に`generated`ディレクトリを削除できない

4. **移行コスト**
   - 既存のインポートパスをすべて変更する必要がある
   - チーム全体での理解と合意が必要

## 推奨事項

### このパターンが適している場合

- 機能ごとにディレクトリが明確に分かれている
- GraphQLクエリとコンポーネントが密接に関連している
- コロケーションを重視する開発スタイル

### 従来のパターン (client) が適している場合

- すべての型定義を一箇所で管理したい
- インポートパスを統一したい
- 生成ファイル数を最小限にしたい

## 結論

`near-operation-file`プリセットは正常に動作し、コロケーションによる開発体験の向上が期待できます。ただし、生成ファイル数の増加とインポートの複雑化がトレードオフとなります。

**推奨**: 現時点では`client`プリセットを維持し、プロジェクトの規模が大きくなり、機能ごとの独立性が重要になった段階で再検討することを推奨します。

## 参考情報

- [GraphQL Code Generator - near-operation-file preset](https://the-guild.dev/graphql/codegen/plugins/presets/near-operation-file-preset)
- ブランチ: `spike/near-operation-file-pattern`
- ビルド: ✅ 成功
- 型チェック: ✅ 成功
