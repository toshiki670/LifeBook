import type { BookCardFragment, BookDetailFragment } from "./fragments.generated"

// BookCardFragmentを使用するコンポーネント
export function BookCard({ book }: { book: BookCardFragment }) {
  return (
    <div className="book-card">
      <h3>{book.title}</h3>
      <p>ID: {book.id}</p>
      {book.author && <p>著者: {book.author}</p>}
      {/* book.description は型エラーになる（BookCardFragmentに含まれていない） */}
    </div>
  )
}

// BookDetailFragmentを使用するコンポーネント
export function BookDetail({ book }: { book: BookDetailFragment }) {
  return (
    <div className="book-detail">
      <h2>{book.title}</h2>
      <p>ID: {book.id}</p>
      {book.author && <p>著者: {book.author}</p>}
      {book.description && <p>説明: {book.description}</p>}
      {book.publishedYear && <p>出版年: {book.publishedYear}</p>}
    </div>
  )
}

// フラグメントの型マスキングをテストする関数
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
  const cardBook: BookCardFragment = fullBook
  console.log("Card book:", cardBook)

  // BookDetailFragmentとして使用（型マスキング）
  const detailBook: BookDetailFragment = fullBook
  console.log("Detail book:", detailBook)

  return { cardBook, detailBook }
}
