import { apolloClient } from "~/lib/apollo-client"
import { UpdateBookDocument } from "./mutations.generated"

/**
 * 書籍を更新
 */
export async function updateBook(input: {
  id: number
  title?: string
  author?: string
  description?: string
  publishedYear?: number
}) {
  try {
    const result = await apolloClient.mutate({
      mutation: UpdateBookDocument,
      variables: input,
    })

    // Apollo Clientのエラーチェック
    if (result.error) {
      return {
        error: result.error,
      }
    }

    // GraphQLエラーのチェック（errorPolicy: "all" の場合、errorsが含まれる可能性がある）
    const fetchResult = result as { errors?: readonly unknown[] }
    if (fetchResult.errors && fetchResult.errors.length > 0) {
      return {
        error: new Error("GraphQL error occurred"),
      }
    }

    return { success: true }
  } catch (error) {
    return {
      error: error instanceof Error ? error : new Error("Failed to update book"),
    }
  }
}
