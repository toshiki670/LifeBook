import { Form, redirect, useNavigate, useNavigation } from "react-router"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { Button } from "~/components/ui/button"
import { DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "~/components/ui/dialog"
import type { Route } from "./+types/page"
import { checkBookExists } from "./api/checkBookExists"
import { deleteBook } from "./api/deleteBook"

export function meta() {
  return [{ title: "Delete Book - LifeBook" }]
}

export async function clientLoader({ params }: Route.ClientLoaderArgs) {
  if (!params.id) {
    throw new Response("Book ID is required", { status: 400 })
  }

  const id = Number.parseInt(params.id, 10)

  if (!Number.isInteger(id)) {
    throw new Response("Book ID is invalid", { status: 400 })
  }

  const exists = await checkBookExists(id)

  if (!exists) {
    throw new Response("Book not found", { status: 404 })
  }

  return { bookId: id }
}

export async function clientAction({ params }: Route.ActionArgs) {
  if (!params.id) {
    return {
      success: false,
      error: "Book ID is required",
    }
  }

  const id = Number.parseInt(params.id, 10)

  if (!Number.isInteger(id)) {
    return {
      success: false,
      error: "Book ID is invalid",
    }
  }

  const result = await deleteBook(id)

  if (result.error) {
    return {
      success: false,
      error: result.error.message || "Failed to delete book",
    }
  }

  return redirect("/books")
}

export default function DeleteBook({ loaderData, actionData }: Route.ComponentProps) {
  const { bookId } = loaderData
  const navigate = useNavigate()
  const navigation = useNavigation()
  const isSubmitting = navigation.state === "submitting"

  return (
    <div className="space-y-6">
      <DialogHeader>
        <DialogTitle>書籍を削除しますか？</DialogTitle>
        <DialogDescription>
          <p className="text-sm">
            ID {bookId} の書籍を削除すると元に戻せません。この操作を続行しますか？
          </p>
        </DialogDescription>
      </DialogHeader>

      {actionData?.success === false && actionData.error && (
        <Alert variant="destructive">
          <AlertDescription>{actionData.error}</AlertDescription>
        </Alert>
      )}

      <DialogFooter>
        <Button
          type="button"
          variant="outline"
          onClick={() => navigate("/books")}
          disabled={isSubmitting}
        >
          キャンセル
        </Button>
        <Form method="post">
          <Button type="submit" variant="destructive" disabled={isSubmitting}>
            {isSubmitting ? "削除中..." : "削除する"}
          </Button>
        </Form>
      </DialogFooter>
    </div>
  )
}
