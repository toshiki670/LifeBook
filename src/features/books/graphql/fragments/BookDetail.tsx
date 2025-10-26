import { Card, CardContent } from "~/components/ui/card"
import type { DetailFragment } from "./fragments.generated"

interface BookDetailProps {
  book: DetailFragment
}

export function BookDetail({ book }: BookDetailProps) {
  return (
    <Card>
      <CardContent className="pt-6">
        <h2 className="text-2xl font-bold">{book.title}</h2>
        <p className="text-muted-foreground mt-1">ID: {book.id}</p>
        {book.author && <p className="mt-2">著者: {book.author}</p>}
        {book.description && <p className="mt-2">{book.description}</p>}
        {book.publishedYear && (
          <p className="text-muted-foreground text-sm mt-1">出版年: {book.publishedYear}</p>
        )}
      </CardContent>
    </Card>
  )
}
