import { useEffect } from "react"
import { Form, Link, useNavigate, useNavigation } from "react-router"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { Button } from "~/components/ui/button"
import { DialogHeader, DialogTitle } from "~/components/ui/dialog"
import { Input } from "~/components/ui/input"
import { Label } from "~/components/ui/label"
import { Textarea } from "~/components/ui/textarea"
import { getBook } from "../detail/api/getBook"
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
  const title = formData.get("title") as string
  const author = formData.get("author") as string
  const description = formData.get("description") as string
  const publishedYear = formData.get("publishedYear") as string

  const bookData = {
    id: parseInt(params.id, 10),
    title: title || undefined,
    author: author || undefined,
    description: description || undefined,
    publishedYear: publishedYear ? parseInt(publishedYear, 10) : undefined,
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
      <DialogHeader>
        <DialogTitle>書籍編集</DialogTitle>
      </DialogHeader>

      {actionData?.success === false && actionData.error && (
        <Alert variant="destructive" className="mb-4">
          <AlertDescription>{actionData.error}</AlertDescription>
        </Alert>
      )}

      <Form method="post" className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="title">
            タイトル <span className="text-destructive">*</span>
          </Label>
          <Input
            id="title"
            name="title"
            type="text"
            required
            placeholder="本のタイトル"
            disabled={isSubmitting}
            defaultValue={book.title}
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="author">著者</Label>
          <Input
            id="author"
            name="author"
            type="text"
            placeholder="著者名"
            disabled={isSubmitting}
            defaultValue={book.author || ""}
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="description">説明</Label>
          <Textarea
            id="description"
            name="description"
            placeholder="本の説明"
            rows={3}
            disabled={isSubmitting}
            defaultValue={book.description || ""}
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="publishedYear">出版年</Label>
          <Input
            id="publishedYear"
            name="publishedYear"
            type="number"
            placeholder="2024"
            disabled={isSubmitting}
            defaultValue={book.publishedYear || ""}
          />
        </div>
        <div className="flex gap-2 justify-end">
          <Button type="button" variant="outline" asChild disabled={isSubmitting}>
            <Link to={`/books/${book.id}`}>キャンセル</Link>
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? "更新中..." : "更新"}
          </Button>
        </div>
      </Form>
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
