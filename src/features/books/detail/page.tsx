import { Link, Outlet, useLocation, useNavigate } from "react-router"
import { AppHeader } from "~/components/common/app-header"
import { Button } from "~/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card"
import { Dialog, DialogContent } from "~/components/ui/dialog"
import type { Route } from "./+types/page"
import { getBook } from "./api/getBook"

export function meta(_: Route.MetaArgs) {
  return [
    { title: "Book Detail - LifeBook" },
    { name: "description", content: "View book details" },
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

export default function BookDetail({ loaderData }: Route.ComponentProps) {
  const { book } = loaderData
  const location = useLocation()
  const navigate = useNavigate()
  const isEditModalOpen = location.pathname === `/books/${book.id}/edit`

  return (
    <>
      <AppHeader breadcrumbs={[{ label: "書籍管理", href: "/books" }, { label: book.title }]} />
      <div className="flex flex-1 flex-col gap-4 p-4">
        <Card>
          <CardHeader>
            <CardTitle>{book.title}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            {book.author && (
              <div>
                <p className="text-sm font-semibold text-muted-foreground">著者</p>
                <p className="text-lg">{book.author}</p>
              </div>
            )}
            {book.description && (
              <div>
                <p className="text-sm font-semibold text-muted-foreground">説明</p>
                <p>{book.description}</p>
              </div>
            )}
            {book.publishedYear && (
              <div>
                <p className="text-sm font-semibold text-muted-foreground">出版年</p>
                <p>{book.publishedYear}</p>
              </div>
            )}
            <div className="flex gap-2 mt-4">
              <Button asChild variant="outline">
                <Link to="/books">一覧に戻る</Link>
              </Button>
              <Button asChild>
                <Link to={`/books/${book.id}/edit`}>編集</Link>
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>

      <Dialog
        open={isEditModalOpen}
        onOpenChange={(open) => {
          if (!open) {
            // モーダルを閉じる際は詳細ページに戻る
            navigate(`/books/${book.id}`)
          }
        }}
      >
        <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
          <Outlet />
        </DialogContent>
      </Dialog>
    </>
  )
}

export function ErrorBoundary({ error }: Route.ErrorBoundaryProps) {
  return (
    <>
      <AppHeader breadcrumbs={[{ label: "書籍管理", href: "/books" }, { label: "エラー" }]} />
      <div className="flex flex-1 flex-col gap-4 p-4">
        <Card>
          <CardContent className="pt-6">
            <h2 className="text-2xl font-bold mb-4">エラーが発生しました</h2>
            <p className="text-muted-foreground">
              {error instanceof Error ? error.message : "不明なエラーが発生しました"}
            </p>
            <Button asChild className="mt-4">
              <Link to="/books">一覧に戻る</Link>
            </Button>
          </CardContent>
        </Card>
      </div>
    </>
  )
}
