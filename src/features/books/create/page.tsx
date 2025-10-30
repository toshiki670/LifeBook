import { useEffect } from "react"
import { useNavigate, useNavigation } from "react-router"
import { Alert, AlertDescription } from "~/components/ui/alert"
import { BookForm } from "../shared/components/BookForm"
import { parseBookFormData } from "../shared/utils/parseBookFormData"
import type { Route } from "./+types/page"
import { createBook } from "./api/createBook"

export function meta() {
  return [
    { title: "Create Book - LifeBook" },
    { name: "description", content: "Create a new book" },
  ]
}

export async function clientAction({ request }: Route.ActionArgs) {
  const formData = await request.formData()
  const bookData = parseBookFormData(formData)
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
      {actionData?.success === false && actionData.error && (
        <Alert variant="destructive" className="mb-4">
          <AlertDescription>{actionData.error}</AlertDescription>
        </Alert>
      )}

      <BookForm
        title="新規書籍作成"
        isSubmitting={isSubmitting}
        submitLabel="作成"
        submittingLabel="作成中..."
        cancelTo="/books"
        initialValues={{}}
      />
    </>
  )
}
