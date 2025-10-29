import { useEffect } from "react"
import { Form, Link, useNavigate, useNavigation } from "react-router"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { Button } from "~/components/ui/button"
import { DialogHeader, DialogTitle } from "~/components/ui/dialog"
import { Input } from "~/components/ui/input"
import { Label } from "~/components/ui/label"
import { Textarea } from "~/components/ui/textarea"
import type { Route } from "./+types/page"
import { createBook } from "./api/createBook"

export function meta() {
  return [
    { title: "Create Book - LifeBook" },
    { name: "description", content: "Create a new book" },
  ]
}

export async function action({ request }: Route.ActionArgs) {
  const formData = await request.formData()
  const title = formData.get("title") as string
  const author = formData.get("author") as string
  const description = formData.get("description") as string
  const publishedYear = formData.get("publishedYear") as string

  const bookData = {
    title,
    author: author || undefined,
    description: description || undefined,
    publishedYear: publishedYear ? parseInt(publishedYear, 10) : undefined,
  }

  const result = await createBook(bookData)
  if (result.error) {
    return {
      success: false,
      error: result.error.message || "Failed to create book",
    }
  }
  return { success: true, redirectTo: "/books" }
}

export default function CreateBook({ actionData }: Route.ComponentProps) {
  const navigation = useNavigation()
  const navigate = useNavigate()
  const isSubmitting = navigation.state === "submitting"

  // 成功時にモーダルを閉じて一覧ページに戻る
  useEffect(() => {
    if (actionData?.success) {
      navigate("/books")
    }
  }, [actionData, navigate])

  return (
    <>
      <DialogHeader>
        <DialogTitle>新規書籍作成</DialogTitle>
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
          />
        </div>
        <div className="flex gap-2 justify-end">
          <Button type="button" variant="outline" asChild disabled={isSubmitting}>
            <Link to="/books">キャンセル</Link>
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? "作成中..." : "作成"}
          </Button>
        </div>
      </Form>
    </>
  )
}
