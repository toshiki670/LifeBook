import { Button } from "~/components/ui/button"
import { Card, CardContent } from "~/components/ui/card"
import type { CardFragment } from "./fragments.generated"

interface BookCardProps {
  book: CardFragment
  onDelete?: (id: number) => void
  isDeleting?: boolean
}

export function BookCard({ book, onDelete, isDeleting }: BookCardProps) {
  return (
    <Card>
      <CardContent className="pt-6">
        <div className="flex justify-between items-start">
          <div className="flex-1">
            <h3 className="text-xl font-semibold">{book.title}</h3>
            {book.author && <p className="text-muted-foreground mt-1">著者: {book.author}</p>}
          </div>
          {onDelete && (
            <Button
              variant="destructive"
              size="sm"
              disabled={isDeleting}
              onClick={() => onDelete(book.id)}
              className="ml-4"
            >
              {isDeleting ? "削除中..." : "削除"}
            </Button>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
