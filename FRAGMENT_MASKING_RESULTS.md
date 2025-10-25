# fragment-masking 検証結果

## 検証日
2025-10-25

## 概要
`near-operation-file`プリセットで`fragment-masking`を有効化し、型安全性の向上を検証しました。

## 実装内容

### 1. フラグメントファイルの作成
`src/features/books/fragments.graphql`を作成：
```graphql
fragment BookCard on BookDto {
  id
  title
  author
}

fragment BookDetail on BookDto {
  id
  title
  author
  description
  publishedYear
}
```

### 2. codegen.ymlの設定
```yaml
config:
  fragmentMasking: true
  mask: true
```

### 3. 生成されたファイル
`src/features/books/fragments.generated.ts`が生成され、以下の内容が含まれています：

```typescript
export type BookCardFragment = { 
  __typename?: 'BookDto', 
  id: number, 
  title: string, 
  author?: string | null 
};

export type BookDetailFragment = { 
  __typename?: 'BookDto', 
  id: number, 
  title: string, 
  author?: string | null, 
  description?: string | null, 
  publishedYear?: number | null 
};
```

## 検証結果

### ✅ 成功した項目

1. **フラグメントの型生成**
   - `BookCardFragment`と`BookDetailFragment`が正しく生成
   - 各フラグメントで定義されたフィールドのみが型に含まれる

2. **型マスキングの動作**
   - `BookCardFragment`では`description`と`publishedYear`にアクセスできない
   - `BookDetailFragment`ではすべてのフィールドにアクセス可能
   - TypeScriptの型チェックで正しく制限される

3. **ビルドの成功**
   - `fragment-masking`有効時でもビルドが成功
   - 型チェック（`pnpm tsc --noEmit`）が通る

4. **コロケーションの維持**
   - フラグメントファイルと同じディレクトリに型定義が生成
   - `near-operation-file`プリセットの利点を維持

### ⚠️ 注意点

1. **useFragment関数の生成**
   - `typed-document-node`プラグインでは`useFragment`関数は生成されない
   - `typescript-react-apollo`プラグインが必要（今回は未使用）

2. **型マスキングの制限**
   - 実行時には全フィールドが存在するが、型レベルで制限
   - 型安全性は向上するが、実行時の動作は変わらない

## 使用例

### フラグメントを使用したコンポーネント

```typescript
// BookCardFragmentを使用（descriptionにアクセス不可）
export function BookCard({ book }: { book: BookCardFragment }) {
  return (
    <div>
      <h3>{book.title}</h3>
      <p>ID: {book.id}</p>
      {book.author && <p>著者: {book.author}</p>}
      {/* book.description は型エラー */}
    </div>
  )
}

// BookDetailFragmentを使用（全フィールドにアクセス可能）
export function BookDetail({ book }: { book: BookDetailFragment }) {
  return (
    <div>
      <h2>{book.title}</h2>
      <p>ID: {book.id}</p>
      {book.author && <p>著者: {book.author}</p>}
      {book.description && <p>説明: {book.description}</p>}
      {book.publishedYear && <p>出版年: {book.publishedYear}</p>}
    </div>
  )
}
```

## メリット

1. **型安全性の向上**
   - フラグメントで定義されたフィールドのみにアクセス可能
   - 意図しないフィールドへのアクセスを防ぐ

2. **コンポーネント間の型契約**
   - フラグメントを介して明確な型の受け渡し
   - コンポーネントの責任範囲が明確

3. **リファクタリングの安全性**
   - フラグメントの変更時に影響範囲が明確
   - 型エラーで問題を早期発見

## 結論

`near-operation-file`プリセットで`fragment-masking`は正常に動作し、以下の利点が確認できました：

- ✅ フラグメントベースの型安全性
- ✅ コロケーションの維持
- ✅ ビルドの成功
- ✅ 型チェックの通過

**推奨**: フラグメントを使用する場合は`fragment-masking`を有効化することを推奨します。型安全性が大幅に向上し、コンポーネント間の型契約が明確になります。

## 参考情報

- 設定: `fragmentMasking: true`, `mask: true`
- 生成ファイル: `src/features/books/fragments.generated.ts`
- テストファイル: `src/features/books/fragment-masking-test.ts`
