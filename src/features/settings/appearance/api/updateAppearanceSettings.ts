import { apolloClient } from "~/lib/apollo-client"
import { UpdateAppearanceSettingsDocument } from "./mutations.generated"

export interface UpdateAppearanceSettingsInput {
  theme: string
}

/**
 * 外観設定を更新
 */
export async function updateAppearanceSettings(input: UpdateAppearanceSettingsInput) {
  const result = await apolloClient.mutate({
    mutation: UpdateAppearanceSettingsDocument,
    variables: input,
  })
  return result.data?.settings?.updateAppearanceSettings
}
