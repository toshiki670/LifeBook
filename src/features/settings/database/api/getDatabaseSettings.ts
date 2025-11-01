import { apolloClient } from "~/lib/apollo-client"
import { GetDatabaseSettingsDocument } from "./queries.generated"

/**
 * データベース設定を取得
 */
export async function getDatabaseSettings() {
  const result = await apolloClient.query({
    query: GetDatabaseSettingsDocument,
  })
  return result.data?.settings?.databaseSettings ?? { databaseDirectory: "" }
}
