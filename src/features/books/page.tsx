import type React from "react"
import { Link, useLoaderData } from "react-router"
import { AppHeader } from "~/components/common/app-header"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { Button } from "~/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card"
import { Input } from "~/components/ui/input"
import { Label } from "~/components/ui/label"
import { Textarea } from "~/components/ui/textarea"
import type { BookDto } from "~/generated/graphql"
import { getDbStatus } from "~/lib/graphql"
import type { Route } from "./+types/page"
import { BookCard } from "./graphql/fragments/BookCard"
import { useBookMutations } from "./graphql/mutations/useBookMutations"
import { useBooks } from "./graphql/queries/useBooks"

export function meta(_: Route.MetaArgs) {
  return [
    { title: "Book Manager - LifeBook" },
    { name: "description", content: "Manage books with SeaORM + GraphQL + Tauri" },
  ]
}

export async function clientLoader() {
  const dbStatus = await getDbStatus()
  return { dbStatus }
}

export default function Books() {
  const { dbStatus } = useLoaderData<typeof clientLoader>()
  const { books, loading, error } = useBooks()
  const {
    createBook,
    deleteBook,
    loading: mutationLoading,
    successMessage,
    errorMessage,
  } = useBookMutations()
  const isLoading = loading || mutationLoading

  const handleCreate = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const formData = new FormData(e.currentTarget)

    const title = formData.get("title") as string
    const author = formData.get("author") as string
    const description = formData.get("description") as string
    const publishedYear = formData.get("publishedYear") as string

    try {
      await createBook({
        variables: {
          title,
          author: author || null,
          description: description || null,
          publishedYear: publishedYear ? parseInt(publishedYear, 10) : null,
        },
      })
      e.currentTarget.reset()
    } catch (err) {
      console.error("Failed to create book:", err)
    }
  }

  const handleDelete = async (id: number) => {
    try {
      await deleteBook({ variables: { id } })
    } catch (err) {
      console.error("Failed to delete book:", err)
    }
  }

  return (
    <>
      <AppHeader breadcrumbs={[{ label: "ライブラリ", href: "#" }, { label: "書籍管理" }]} />
      <div className="flex flex-1 flex-col gap-4 p-4">
        <div className="mb-4">
          <h2 className="text-2xl font-bold">Books</h2>
          <p className="text-sm text-muted-foreground mt-2">
            DB Status: <span className="font-semibold">{dbStatus}</span>
          </p>
        </div>

        {errorMessage && (
          <Alert variant="destructive" className="mb-4">
            <AlertDescription>{errorMessage}</AlertDescription>
          </Alert>
        )}

        {successMessage && (
          <Alert className="mb-4">
            <AlertDescription>{successMessage}</AlertDescription>
          </Alert>
        )}

        {error && (
          <Alert variant="destructive" className="mb-4">
            <AlertDescription>データの読み込みに失敗しました: {error.message}</AlertDescription>
          </Alert>
        )}

        {/* 新しい本を追加するフォーム */}
        <Card className="mb-8">
          <CardHeader>
            <CardTitle>新しい本を追加</CardTitle>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleCreate} className="space-y-4">
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
                  disabled={isLoading}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="author">著者</Label>
                <Input
                  id="author"
                  name="author"
                  type="text"
                  placeholder="著者名"
                  disabled={isLoading}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="description">説明</Label>
                <Textarea
                  id="description"
                  name="description"
                  placeholder="本の説明"
                  rows={3}
                  disabled={isLoading}
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="publishedYear">出版年</Label>
                <Input
                  id="publishedYear"
                  name="publishedYear"
                  type="number"
                  placeholder="2024"
                  disabled={isLoading}
                />
              </div>
              <Button type="submit" className="w-full" disabled={isLoading}>
                {mutationLoading ? "追加中..." : "追加"}
              </Button>
            </form>
          </CardContent>
        </Card>

        {/* 本のリスト */}
        <Card>
          <CardHeader>
            <CardTitle>本のリスト</CardTitle>
          </CardHeader>
          <CardContent>
            {loading ? (
              <p className="text-muted-foreground">読み込み中...</p>
            ) : books.length === 0 ? (
              <p className="text-muted-foreground">
                本がまだありません。上のフォームから追加してください。
              </p>
            ) : (
              <div className="space-y-4">
                {books.map((book: BookDto) => (
                  <BookCard
                    key={book.id}
                    book={book}
                    onDelete={handleDelete}
                    isDeleting={mutationLoading}
                  />
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </>
  )
}

export function ErrorBoundary({ error }: Route.ErrorBoundaryProps) {
  return (
    <>
      <AppHeader breadcrumbs={[{ label: "ライブラリ", href: "#" }, { label: "書籍管理" }]} />
      <div className="flex flex-1 flex-col gap-4 p-4">
        <div className="mb-4">
          <h2 className="text-2xl font-bold">エラーが発生しました</h2>
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
    </>
  )
}
