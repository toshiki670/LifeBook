import { useMutation } from "@apollo/client/react"
import { useState } from "react"
import { CreateDocument, DeleteDocument, UpdateDocument } from "./mutations.generated"

/**
 * 書籍のミューテーション操作をまとめたカスタムフック
 */
export function useBookMutations() {
  const [successMessage, setSuccessMessage] = useState<string | null>(null)
  const [errorMessage, setErrorMessage] = useState<string | null>(null)

  const [createBook, { loading: creating }] = useMutation(CreateDocument, {
    onCompleted: () => setSuccessMessage("本を追加しました"),
    onError: (error) => setErrorMessage(error.message),
    refetchQueries: ["GetAll"],
  })

  const [updateBook, { loading: updating }] = useMutation(UpdateDocument, {
    onCompleted: () => setSuccessMessage("本を更新しました"),
    onError: (error) => setErrorMessage(error.message),
  })

  const [deleteBook, { loading: deleting }] = useMutation(DeleteDocument, {
    onCompleted: () => setSuccessMessage("本を削除しました"),
    onError: (error) => setErrorMessage(error.message),
    refetchQueries: ["GetAll"],
  })

  return {
    createBook,
    updateBook,
    deleteBook,
    loading: creating || updating || deleting,
    successMessage,
    errorMessage,
    clearMessages: () => {
      setSuccessMessage(null)
      setErrorMessage(null)
    },
  }
}
