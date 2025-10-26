# GraphQL Code Generator 設定の追加

PR #39 を参考に、near-operation-file プリセットを使用した GraphQL Code Generator の環境を構築します。

## 実装手順

### 1. ブランチ作成

新しいブランチ `feature/graphql-codegen-setup` を作成します。

### 2. package.json の更新

#### devDependencies に追加するパッケージ:

- `@graphql-codegen/cli`: GraphQL Code Generator のメインツール
- `@graphql-codegen/typescript`: 基本型定義生成用プラグイン
- `@graphql-codegen/typescript-operations`: クエリ/ミューテーション型生成用
- `@graphql-codegen/near-operation-file-preset`: 各 GraphQL ファイルの近くに型を生成
- `@graphql-codegen/typed-document-node`: TypedDocumentNode 生成用
- `graphql`: GraphQL のコアライブラリ

#### scripts に追加:

```json
"codegen": "pnpm export-schema && graphql-codegen"
```

これにより、スキーマをエクスポートしてから型生成を行います。

### 3. codegen.yml の作成

プロジェクトルートに以下の設定で作成:

```yaml
overwrite: true
schema: "schema.graphql"
documents: "src/**/*.graphql"
generates:
  src/generated/graphql.ts:
    plugins:
      - add:
          content: "// @ts-nocheck"
      - typescript
  src/:
    preset: "near-operation-file"
    presetConfig:
      baseTypesPath: generated/graphql.ts
      extension: .generated.ts
    plugins:
      - add:
          content: "// @ts-nocheck"
      - typescript-operations
      - typed-document-node
    config:
      useTypeImports: true
      skipTypename: false
      documentMode: "documentNode"
      fragmentMasking: true
```

**設定の説明:**

- `overwrite: true`: 既存ファイルを上書き
- `schema`: バックエンドから出力された schema.graphql を参照
- `documents`: src 配下のすべての.graphql ファイルを対象
- 共通型定義は `src/generated/graphql.ts` に集約
- 各 GraphQL ファイルの近くに `.generated.ts` を生成
- Fragment Masking を有効化して型安全性を向上

### 4. biome.jsonc の更新

生成ファイルを lint/format の対象外にするため、`files.includes`に追加:

```jsonc
"includes": ["**", "!**/gen/schemas", "!src/generated", "!**/*.generated.ts"]
```

### 5. Apollo Client 設定の追加

#### vite.config.ts に追加:

```typescript
optimizeDeps: {
  include: ["@apollo/client"],
},
```

#### src/lib/apollo-client.ts を作成:

Tauri invoke 経由で GraphQL リクエストを実行するカスタム Apollo Client を実装。

#### src/root.tsx を更新:

ApolloProvider でアプリ全体をラップ。

### 6. package.json に dependencies 追加

- `@apollo/client`
- `@graphql-typed-document-node/core`
- `graphql-tag`

## 想定される生成ファイル構造

```
src/
├── generated/
│   └── graphql.ts (共通型定義)
└── features/
    ├── books/
    │   ├── queries.graphql
    │   ├── queries.generated.ts ← 生成される
    │   ├── mutations.graphql
    │   └── mutations.generated.ts ← 生成される
    └── settings/
        ├── queries.graphql
        ├── queries.generated.ts ← 生成される
        ├── mutations.graphql
        └── mutations.generated.ts ← 生成される
```

## 注意事項

- 実際の GraphQL ファイル(queries.graphql, mutations.graphql)の作成は別タスク
- pnpm install は実行しない（ユーザーが手動で実行）
- codegen の実行も行わない（設定のみを追加）

## 実装結果

- ✅ package.json の更新
- ✅ codegen.yml の作成
- ✅ biome.jsonc の更新
- ✅ Apollo Client 設定の追加
- ✅ vite.config.ts の更新
- ✅ src/root.tsx の更新
- ✅ ドキュメント更新

PR #44 にて実装完了。
