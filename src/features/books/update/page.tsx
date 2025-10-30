import { useEffect } from "react"
import { Link, useNavigate, useNavigation } from "react-router"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { Button } from "~/components/ui/button"
import { getBook } from "../detail/api/getBook"
import { BookForm } from "../shared/components/BookForm"
import { parseBookFormData } from "../shared/utils/parseBookFormData"
import type { Route } from "./+types/page"
import { updateBook } from "./api/updateBook"

export function meta() {
  return [
    { title: "Update Book - LifeBook" },
    { name: "description", content: "Update book information" },
  ]
}

export async function clientLoader({ params }: Route.ClientLoaderArgs) {
  if (!params.id) {
    throw new Response("Book ID is required", { status: 400 })
  }
  const book = await getBook(parseInt(params.id, 10))

  if (!book) {
    throw new Response("Book not found", { status: 404 })
  }

  return { book }
}

export async function clientAction({ request, params }: Route.ActionArgs) {
  if (!params.id) {
    return {
      success: false,
      error: "Book ID is required",
    }
  }

  const formData = await request.formData()
  const parsed = parseBookFormData(formData)
  const bookData = {
    id: parseInt(params.id, 10),
    ...parsed,
  }

  const result = await updateBook(bookData)
  if (result.error) {
    return {
      success: false,
      error: result.error.message || "Failed to update book",
    }
  }
  return { success: true, redirectTo: `/books/${params.id}` }
}

export default function UpdateBook({ loaderData, actionData }: Route.ComponentProps) {
  const navigation = useNavigation()
  const navigate = useNavigate()
  const isSubmitting = navigation.state === "submitting"
  const { book } = loaderData

  // 成功時にモーダルを閉じて詳細ページに戻る
  useEffect(() => {
    if (actionData?.success) {
      navigate(`/books/${book.id}`)
    }
  }, [actionData, navigate, book.id])

  return (
    <>
      {actionData?.success === false && actionData.error && (
        <Alert variant="destructive" className="mb-4">
          <AlertDescription>{actionData.error}</AlertDescription>
        </Alert>
      )}

      <BookForm
        title="書籍編集"
        isSubmitting={isSubmitting}
        submitLabel="更新"
        submittingLabel="更新中..."
        cancelTo={`/books/${book.id}`}
        initialValues={{
          title: book.title,
          author: book.author || undefined,
          description: book.description || undefined,
          publishedYear: book.publishedYear || undefined,
        }}
      />
    </>
  )
}

export function ErrorBoundary({ error }: Route.ErrorBoundaryProps) {
  return (
    <div className="p-4">
      <h2 className="text-xl font-bold mb-4">エラーが発生しました</h2>
      <p className="text-muted-foreground mb-4">
        {error instanceof Error ? error.message : "不明なエラーが発生しました"}
      </p>
      <Button asChild>
        <Link to="/books">一覧に戻る</Link>
      </Button>
    </div>
  )
}
