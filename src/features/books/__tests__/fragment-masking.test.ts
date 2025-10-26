import type { CardFragment, DetailFragment } from "../graphql/fragments/fragments.generated"

// fragment-maskingの動作テスト
export function testFragmentMasking() {
  // BookDtoの完全なデータ
  const fullBook = {
    id: 1,
    title: "テスト本",
    author: "テスト著者",
    description: "テスト説明",
    publishedYear: 2024,
  }

  // BookCardFragmentとして使用（型マスキング）
  const cardBook: CardFragment = fullBook
  console.log("Card book:", cardBook)

  // BookCardFragmentでは description にアクセスできない（型エラー）
  // console.log(cardBook.description) // ← これは型エラーになる

  // BookDetailFragmentとして使用（型マスキング）
  const detailBook: DetailFragment = fullBook
  console.log("Detail book:", detailBook)

  // BookDetailFragmentでは description にアクセスできる
  console.log("Description:", detailBook.description)

  return { cardBook, detailBook }
}

// 型マスキングの効果を確認する関数
export function demonstrateTypeMasking() {
  const book = {
    id: 1,
    title: "サンプル本",
    author: "サンプル著者",
    description: "サンプル説明",
    publishedYear: 2023,
  }

  // BookCardFragmentとして型を制限
  const cardBook: CardFragment = book

  // 以下のプロパティはアクセス可能
  console.log("ID:", cardBook.id)
  console.log("Title:", cardBook.title)
  console.log("Author:", cardBook.author)

  // 以下のプロパティは型エラー（実際には実行時には存在するが、型レベルで制限）
  // console.log("Description:", cardBook.description) // TypeScript エラー
  // console.log("Published Year:", cardBook.publishedYear) // TypeScript エラー

  return cardBook
}
