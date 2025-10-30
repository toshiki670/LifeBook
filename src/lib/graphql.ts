import { invoke } from "@tauri-apps/api/core"

/**
 * データベース接続状態を確認
 */
export async function getDbStatus(): Promise<string> {
  return invoke<string>("get_db_status")
}

// Settings types
export interface GeneralSettings {
  language: string
}

export interface AppearanceSettings {
  theme: string
}

export interface DatabaseSettings {
  databaseDirectory: string
}

/**
 * 一般設定を取得
 */
export async function getGeneralSettings() {
  const query = `
    query {
      settings {
        generalSettings {
          language
        }
      }
    }
  `
  const result = await executeGraphQL<{ settings: { generalSettings: GeneralSettings } }>(query)
  return {
    data: result.data ? { generalSettings: result.data.settings.generalSettings } : undefined,
    errors: result.errors,
  }
}

/**
 * 外観設定を取得
 */
export async function getAppearanceSettings() {
  const query = `
    query {
      settings {
        appearanceSettings {
          theme
        }
      }
    }
  `
  const result = await executeGraphQL<{ settings: { appearanceSettings: AppearanceSettings } }>(
    query,
  )
  return {
    data: result.data ? { appearanceSettings: result.data.settings.appearanceSettings } : undefined,
    errors: result.errors,
  }
}

/**
 * データベース設定を取得
 */
export async function getDatabaseSettings() {
  const query = `
    query {
      settings {
        databaseSettings {
          databaseDirectory
        }
      }
    }
  `
  const result = await executeGraphQL<{ settings: { databaseSettings: DatabaseSettings } }>(query)
  return {
    data: result.data ? { databaseSettings: result.data.settings.databaseSettings } : undefined,
    errors: result.errors,
  }
}

/**
 * 一般設定を更新
 */
export async function updateGeneralSettings(language: string) {
  const query = `
    mutation UpdateGeneralSettings($language: String) {
      settings {
        updateGeneralSettings(language: $language) {
          language
        }
      }
    }
  `
  const variables = { language }
  const result = await executeGraphQL<{ settings: { updateGeneralSettings: GeneralSettings } }>(
    query,
    variables,
  )
  return {
    data: result.data
      ? { updateGeneralSettings: result.data.settings.updateGeneralSettings }
      : undefined,
    errors: result.errors,
  }
}

/**
 * 外観設定を更新
 */
export async function updateAppearanceSettings(theme: string) {
  const query = `
    mutation UpdateAppearanceSettings($theme: String) {
      settings {
        updateAppearanceSettings(theme: $theme) {
          theme
        }
      }
    }
  `
  const variables = { theme }
  const result = await executeGraphQL<{
    settings: { updateAppearanceSettings: AppearanceSettings }
  }>(query, variables)
  return {
    data: result.data
      ? { updateAppearanceSettings: result.data.settings.updateAppearanceSettings }
      : undefined,
    errors: result.errors,
  }
}

/**
 * データベース設定を更新
 */
export async function updateDatabaseSettings(databaseDirectory: string) {
  const query = `
    mutation UpdateDatabaseSettings($databaseDirectory: String) {
      settings {
        updateDatabaseSettings(databaseDirectory: $databaseDirectory) {
          databaseDirectory
        }
      }
    }
  `
  const variables = { databaseDirectory }
  const result = await executeGraphQL<{ settings: { updateDatabaseSettings: DatabaseSettings } }>(
    query,
    variables,
  )
  return {
    data: result.data
      ? { updateDatabaseSettings: result.data.settings.updateDatabaseSettings }
      : undefined,
    errors: result.errors,
  }
}
