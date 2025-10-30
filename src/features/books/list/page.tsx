import { Link, Outlet, useLocation, useNavigate } from "react-router"
import { AppHeader } from "~/components/common/app-header"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { Button } from "~/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card"
import { Dialog, DialogContent } from "~/components/ui/dialog"
import { getDbStatus } from "~/lib/graphql"
import type { Route } from "./+types/page"
import { getAllBooks } from "./api/getAllBooks"
import { BookList } from "./BookList"

export function meta(_: Route.MetaArgs) {
  return [
    { title: "Book Manager - LifeBook" },
    {
      name: "description",
      content: "Manage books with SeaORM + GraphQL + Tauri",
    },
  ]
}

export async function clientLoader() {
  const [books, dbStatus] = await Promise.all([getAllBooks(), getDbStatus()])

  return {
    books,
    dbStatus,
  }
}

export default function Books({ loaderData }: Route.ComponentProps) {
  const { books, dbStatus } = loaderData
  const location = useLocation()
  const navigate = useNavigate()
  const isCreateModalOpen = location.pathname === "/books/create"

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

        <div className="mb-4">
          <Button asChild>
            <Link to="/books/create">新規作成</Link>
          </Button>
        </div>

        <Card>
          <CardHeader>
            <CardTitle>本のリスト</CardTitle>
          </CardHeader>
          <CardContent>
            <BookList books={books} />
          </CardContent>
        </Card>
      </div>

      <Dialog
        open={isCreateModalOpen}
        onOpenChange={(open) => {
          if (!open) {
            // モーダルを閉じる際は一覧ページに戻る
            navigate("/books")
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
