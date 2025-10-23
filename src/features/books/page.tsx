import React from "react"
import { Link, useLoaderData } from "react-router"
import { AppHeader } from "~/components/common/app-header"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { Button } from "~/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card"
import { Input } from "~/components/ui/input"
import { Label } from "~/components/ui/label"
import { Textarea } from "~/components/ui/textarea"
import { useCreateBookMutation, useDeleteBookMutation, useGetBooksQuery } from "~/generated/graphql"
import { getDbStatus } from "~/lib/graphql"
import type { Route } from "./+types/page"

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
  const { data, loading, error, refetch } = useGetBooksQuery()
  const [createBook, { loading: creating }] = useCreateBookMutation()
  const [deleteBook, { loading: deleting }] = useDeleteBookMutation()
  const [successMessage, setSuccessMessage] = React.useState<string | null>(null)
  const [errorMessage, setErrorMessage] = React.useState<string | null>(null)

  const books = data?.library?.books || []
  const isLoading = loading || creating || deleting

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
      setSuccessMessage("本を追加しました")
      setErrorMessage(null)
      e.currentTarget.reset()
      refetch()
    } catch (err) {
      setErrorMessage(err instanceof Error ? err.message : "本の追加に失敗しました")
      setSuccessMessage(null)
    }
  }

  const handleDelete = async (id: number) => {
    try {
      await deleteBook({ variables: { id } })
      setSuccessMessage("本を削除しました")
      setErrorMessage(null)
      refetch()
    } catch (err) {
      setErrorMessage(err instanceof Error ? err.message : "本の削除に失敗しました")
      setSuccessMessage(null)
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
                {creating ? "追加中..." : "追加"}
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
                {books.map(
                  (book: {
                    id: number
                    title: string
                    author?: string | null
                    description?: string | null
                    publishedYear?: number | null
                  }) => (
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
                          <Button
                            variant="destructive"
                            size="sm"
                            disabled={isLoading}
                            onClick={() => handleDelete(book.id)}
                            className="ml-4"
                          >
                            {deleting ? "削除中..." : "削除"}
                          </Button>
                        </div>
                      </CardContent>
                    </Card>
                  ),
                )}
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
