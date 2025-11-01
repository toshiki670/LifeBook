import { apolloClient } from "~/lib/apollo-client"
import { DeleteBookDocument } from "./mutations.generated"

/**
 * 書籍を削除
 */
export async function deleteBook(id: number) {
  try {
    const result = await apolloClient.mutate({
      mutation: DeleteBookDocument,
      variables: { id },
    })

    if (result.error) {
      return {
        error: result.error,
      }
    }

    const fetchResult = result as { errors?: readonly unknown[] }
    if (fetchResult.errors && fetchResult.errors.length > 0) {
      return {
        error: new Error("GraphQL error occurred"),
      }
    }

    return { success: true }
  } catch (error) {
    return {
      error: error instanceof Error ? error : new Error("Failed to delete book"),
    }
  }
}

