import { apolloClient } from "~/lib/apollo-client"
import { UpdateDatabaseSettingsDocument } from "./mutations.generated"

export interface UpdateDatabaseSettingsInput {
  databaseDirectory: string
}

/**
 * データベース設定を更新
 */
export async function updateDatabaseSettings(input: UpdateDatabaseSettingsInput) {
  const result = await apolloClient.mutate({
    mutation: UpdateDatabaseSettingsDocument,
    variables: input,
  })
  return result.data?.settings?.updateDatabaseSettings
}
