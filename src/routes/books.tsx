import { Form, Link, useActionData, useLoaderData, useNavigation } from "react-router"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { Button } from "~/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card"
import { Input } from "~/components/ui/input"
import { Label } from "~/components/ui/label"
import { Textarea } from "~/components/ui/textarea"
import { createBook, deleteBook, getBooks, getDbStatus } from "../lib/graphql"
import type { Route } from "./+types/books"

export function meta(_: Route.MetaArgs) {
  return [
    { title: "Book Manager - LifeBook" },
    { name: "description", content: "Manage books with SeaORM + GraphQL + Tauri" },
  ]
}

export async function clientLoader() {
  const [booksResponse, dbStatus] = await Promise.all([getBooks(), getDbStatus()])

  if (booksResponse.errors) {
    throw new Error(booksResponse.errors[0]?.message || "Failed to load books")
  }

  return {
    books: booksResponse.data?.books || [],
    dbStatus,
  }
}

export async function clientAction({ request }: Route.ClientActionArgs) {
  const formData = await request.formData()
  const intent = formData.get("intent")

  if (intent === "create") {
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

    const response = await createBook(bookData)
    if (response.errors) {
      return { success: false, error: response.errors[0]?.message || "Failed to create book" }
    }
    return { success: true, message: "本を追加しました" }
  }

  if (intent === "delete") {
    const id = parseInt(formData.get("id") as string, 10)
    const response = await deleteBook(id)
    if (response.errors) {
      return { success: false, error: response.errors[0]?.message || "Failed to delete book" }
    }
    return { success: true, message: "本を削除しました" }
  }

  return { success: false, error: "Unknown action" }
}

export default function Books() {
  const { books, dbStatus } = useLoaderData<typeof clientLoader>()
  const actionData = useActionData<typeof clientAction>()
  const navigation = useNavigation()

  const isSubmitting = navigation.state === "submitting"
  const isLoading = navigation.state === "loading"

  return (
    <div className="container mx-auto p-8">
      <div className="mb-8">
        <div className="flex items-center justify-between mb-4">
          <h1 className="text-4xl font-bold">LifeBook - GraphQL Demo</h1>
          <Button asChild variant="outline">
            <Link to="/">← ホームへ戻る</Link>
          </Button>
        </div>
        <p className="text-muted-foreground">SeaORM + GraphQL + Tauri の統合デモ</p>
        <p className="text-sm text-muted-foreground mt-2">
          DB Status: <span className="font-semibold">{dbStatus}</span>
        </p>
      </div>

      {actionData?.success === false && actionData.error && (
        <Alert variant="destructive" className="mb-4">
          <AlertDescription>{actionData.error}</AlertDescription>
        </Alert>
      )}

      {actionData?.success && actionData.message && (
        <Alert className="mb-4">
          <AlertDescription>{actionData.message}</AlertDescription>
        </Alert>
      )}

      {/* 新しい本を追加するフォーム */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle>新しい本を追加</CardTitle>
        </CardHeader>
        <CardContent>
          <Form method="post" className="space-y-4">
            <input type="hidden" name="intent" value="create" />
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
            <Button type="submit" className="w-full" disabled={isSubmitting}>
              {isSubmitting ? "追加中..." : "追加"}
            </Button>
          </Form>
        </CardContent>
      </Card>

      {/* 本のリスト */}
      <Card>
        <CardHeader>
          <CardTitle>本のリスト</CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <p className="text-muted-foreground">読み込み中...</p>
          ) : books.length === 0 ? (
            <p className="text-muted-foreground">
              本がまだありません。上のフォームから追加してください。
            </p>
          ) : (
            <div className="space-y-4">
              {books.map((book) => (
                <Card key={book.id}>
                  <CardContent className="pt-6">
                    <div className="flex justify-between items-start">
                      <div className="flex-1">
                        <h3 className="text-xl font-semibold">{book.title}</h3>
                        {book.author && (
                          <p className="text-muted-foreground mt-1">著者: {book.author}</p>
                        )}
                        {book.description && <p className="mt-2">{book.description}</p>}
                        {book.publishedYear && (
                          <p className="text-muted-foreground text-sm mt-1">
                            出版年: {book.publishedYear}
                          </p>
                        )}
                      </div>
                      <Form method="post" className="ml-4">
                        <input type="hidden" name="intent" value="delete" />
                        <input type="hidden" name="id" value={book.id} />
                        <Button
                          type="submit"
                          variant="destructive"
                          size="sm"
                          disabled={isSubmitting}
                        >
                          {isSubmitting ? "削除中..." : "削除"}
                        </Button>
                      </Form>
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}

export function ErrorBoundary({ error }: Route.ErrorBoundaryProps) {
  return (
    <div className="container mx-auto p-8">
      <div className="mb-8">
        <div className="flex items-center justify-between mb-4">
          <h1 className="text-4xl font-bold">エラーが発生しました</h1>
          <Button asChild variant="outline">
            <Link to="/">← ホームへ戻る</Link>
          </Button>
        </div>
      </div>

      <Alert variant="destructive">
        <AlertDescription>
          <p className="font-semibold mb-2">データの読み込みに失敗しました</p>
          <p className="text-sm">
            {error instanceof Error ? error.message : "不明なエラーが発生しました"}
          </p>
        </AlertDescription>
      </Alert>

      <div className="mt-4">
        <Button asChild>
          <Link to="/books" reloadDocument>
            再読み込み
          </Link>
        </Button>
      </div>
    </div>
  )
}
