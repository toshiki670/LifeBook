import { useNavigation } from "react-router"
import type { BookCardFragment } from "../shared/fragments/book.generated"
import { BookCard } from "./BookCard"

interface BookListProps {
  books: BookCardFragment[]
}

export function BookList({ books }: BookListProps) {
  const navigation = useNavigation()
  const isLoading = navigation.state === "loading"
  const deleteTargetMatch = navigation.location?.pathname?.match(/^\/books\/(\d+)\/delete$/)
  const deletingId = deleteTargetMatch ? Number.parseInt(deleteTargetMatch[1], 10) : null
  const isNavigatingToDelete = navigation.state !== "idle" && Number.isInteger(deletingId)

  if (isLoading) {
    return <p className="text-muted-foreground">読み込み中...</p>
  }

  if (books.length === 0) {
    return (
      <p className="text-muted-foreground">
        本がまだありません。上のフォームから追加してください。
      </p>
    )
  }

  return (
    <div className="space-y-4">
      {books.map((book) => {
        const isNavigating = isNavigatingToDelete && deletingId === book.id
        return <BookCard key={book.id} book={book} isNavigating={isNavigating} />
      })}
    </div>
  )
}
