import { useQuery } from "@apollo/client/react"
import { GetAllDocument } from "./queries.generated"

/**
 * すべての書籍を取得するカスタムフック
 */
export function useBooks() {
  const { data, loading, error, refetch } = useQuery(GetAllDocument)

  return {
    books: data?.library?.books || [],
    loading,
    error,
    refetch,
    isEmpty: !loading && data?.library?.books.length === 0,
  }
}
