import { Link } from "react-router"
import { Button } from "~/components/ui/button"
import { Card, CardContent } from "~/components/ui/card"
import type { BookCardFragment } from "../shared/fragments/book.generated"

interface BookCardProps {
  book: BookCardFragment
  isNavigating: boolean
}

export function BookCard({ book, isNavigating }: BookCardProps) {
  return (
    <Card>
      <CardContent className="pt-6">
        <div className="flex justify-between items-start">
          <Link to={`/books/${book.id}`} className="flex-1">
            <h3 className="text-xl font-semibold hover:underline">{book.title}</h3>
            {book.author && <p className="text-muted-foreground mt-1">著者: {book.author}</p>}
          </Link>
          <div className="ml-4">
            <Button asChild variant="destructive" size="sm" disabled={isNavigating}>
              <Link to={`/books/${book.id}/delete`}>削除</Link>
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
