import { apolloClient } from "~/lib/apollo-client"
import type { BookCardFragment } from "../../shared/fragments/book.generated"
import { GetAllBooksDocument } from "./queries.generated"

/**
 * すべての書籍を取得
 */
export async function getAllBooks(): Promise<BookCardFragment[]> {
  const result = await apolloClient.query({
    query: GetAllBooksDocument,
  })
  return (result.data?.library.books as BookCardFragment[]) || []
}
