import { apolloClient } from "~/lib/apollo-client"
import { UpdateGeneralSettingsDocument } from "./mutations.generated"

export interface UpdateGeneralSettingsInput {
  language: string
}

/**
 * 一般設定を更新
 */
export async function updateGeneralSettings(input: UpdateGeneralSettingsInput) {
  const result = await apolloClient.mutate({
    mutation: UpdateGeneralSettingsDocument,
    variables: input,
  })
  return result.data?.settings?.updateGeneralSettings
}
