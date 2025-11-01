import { apolloClient } from "~/lib/apollo-client"
import { GetAppearanceSettingsDocument } from "./queries.generated"

/**
 * 外観設定を取得
 */
export async function getAppearanceSettings() {
  const result = await apolloClient.query({
    query: GetAppearanceSettingsDocument,
  })
  return result.data?.settings?.appearanceSettings ?? { theme: "system" }
}
