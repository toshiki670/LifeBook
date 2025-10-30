import { apolloClient } from "~/lib/apollo-client"
import type { BookDetailFragment } from "../../shared/fragments/book.generated"
import { GetBookDocument } from "./queries.generated"

/**
 * IDで書籍を取得
 */
export async function getBook(id: number): Promise<BookDetailFragment | null> {
  const result = await apolloClient.query({
    query: GetBookDocument,
    variables: { id },
  })
  return (result.data?.library?.book as BookDetailFragment | undefined) || null
}
