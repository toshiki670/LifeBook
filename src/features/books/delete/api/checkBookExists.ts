import { apolloClient } from "~/lib/apollo-client"
import { CheckBookExistsDocument } from "./queries.generated"

/**
 * 指定した書籍が存在するか確認
 */
export async function checkBookExists(id: number): Promise<boolean> {
  const result = await apolloClient.query({
    query: CheckBookExistsDocument,
    variables: { id },
    fetchPolicy: "network-only",
  })

  return Boolean(result.data?.library?.book)
}
