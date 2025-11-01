import { apolloClient } from "~/lib/apollo-client"
import { GetGeneralSettingsDocument } from "./queries.generated"

/**
 * 一般設定を取得
 */
export async function getGeneralSettings() {
  const result = await apolloClient.query({
    query: GetGeneralSettingsDocument,
  })
  return result.data?.settings?.generalSettings ?? { language: "ja" }
}
