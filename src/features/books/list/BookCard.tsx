import { Form, Link } from "react-router"
import { Button } from "~/components/ui/button"
import { Card, CardContent } from "~/components/ui/card"
import type { BookCardFragment } from "../shared/fragments/book.generated"

interface BookCardProps {
  book: BookCardFragment
  isSubmitting: boolean
}

export function BookCard({ book, isSubmitting }: BookCardProps) {
  return (
    <Card>
      <CardContent className="pt-6">
        <div className="flex justify-between items-start">
          <Link to={`/books/${book.id}`} className="flex-1">
            <h3 className="text-xl font-semibold hover:underline">{book.title}</h3>
            {book.author && <p className="text-muted-foreground mt-1">著者: {book.author}</p>}
          </Link>
          <Form method="post" className="ml-4">
            <input type="hidden" name="intent" value="delete" />
            <input type="hidden" name="id" value={book.id} />
            <Button type="submit" variant="destructive" size="sm" disabled={isSubmitting}>
              {isSubmitting ? "削除中..." : "削除"}
            </Button>
          </Form>
        </div>
      </CardContent>
    </Card>
  )
}
