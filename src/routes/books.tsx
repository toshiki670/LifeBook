import { Form, Link, useActionData, useLoaderData, useNavigation } from "react-router"
import { BookOpen } from "lucide-react"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { Button } from "~/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card"
import { Input } from "~/components/ui/input"
import { Label } from "~/components/ui/label"
import { Textarea } from "~/components/ui/textarea"
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarTrigger,
} from "~/components/ui/sidebar"
import { Separator } from "~/components/ui/separator"
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
    <SidebarProvider>
      <Sidebar collapsible="icon">
        <SidebarHeader>
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton size="lg" asChild>
                <Link to="/">
                  <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                    <BookOpen className="size-4" />
                  </div>
                  <div className="flex flex-col gap-0.5 leading-none">
                    <span className="font-semibold">LifeBook</span>
                    <span className="text-xs">v0.1.0</span>
                  </div>
                </Link>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>ライブラリ</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton asChild isActive>
                    <Link to="/">
                      <BookOpen className="size-4" />
                      <span>Books</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <SidebarTrigger />
          <Separator orientation="vertical" className="mr-2 h-4" />
          <div className="flex flex-1 items-center justify-between">
            <div>
              <h1 className="text-lg font-semibold">LifeBook - GraphQL Demo</h1>
              <p className="text-xs text-muted-foreground">
                DB Status: <span className="font-semibold">{dbStatus}</span>
              </p>
            </div>
          </div>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4">
          {actionData?.success === false && actionData.error && (
            <Alert variant="destructive">
              <AlertDescription>{actionData.error}</AlertDescription>
            </Alert>
          )}

          {actionData?.success && actionData.message && (
            <Alert>
              <AlertDescription>{actionData.message}</AlertDescription>
            </Alert>
          )}

          {/* 新しい本を追加するフォーム */}
          <Card>
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
      </SidebarInset>
    </SidebarProvider>
  )
}

export function ErrorBoundary({ error }: Route.ErrorBoundaryProps) {
  return (
    <SidebarProvider>
      <Sidebar collapsible="icon">
        <SidebarHeader>
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton size="lg" asChild>
                <Link to="/">
                  <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                    <BookOpen className="size-4" />
                  </div>
                  <div className="flex flex-col gap-0.5 leading-none">
                    <span className="font-semibold">LifeBook</span>
                    <span className="text-xs">v0.1.0</span>
                  </div>
                </Link>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>ライブラリ</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton asChild isActive>
                    <Link to="/">
                      <BookOpen className="size-4" />
                      <span>Books</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <SidebarTrigger />
          <Separator orientation="vertical" className="mr-2 h-4" />
          <div className="flex flex-1 items-center justify-between">
            <h1 className="text-lg font-semibold">エラーが発生しました</h1>
          </div>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4">
          <Alert variant="destructive">
            <AlertDescription>
              <p className="font-semibold mb-2">データの読み込みに失敗しました</p>
              <p className="text-sm">
                {error instanceof Error ? error.message : "不明なエラーが発生しました"}
              </p>
            </AlertDescription>
          </Alert>

          <div>
            <Button asChild>
              <Link to="/" reloadDocument>
                再読み込み
              </Link>
            </Button>
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
  )
}
