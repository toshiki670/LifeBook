// Settings Feature - Custom Hook for Settings Management

import { useCallback, useState } from "react"
import {
  type AppearanceSettings,
  type DatabaseSettings,
  type GeneralSettings,
  getAppearanceSettings,
  getDatabaseSettings,
  getGeneralSettings,
  updateAppearanceSettings as updateAppearanceSettingsApi,
  updateDatabaseSettings as updateDatabaseSettingsApi,
  updateGeneralSettings as updateGeneralSettingsApi,
} from "~/lib/graphql"

interface UseSettingsReturn {
  // General Settings
  generalSettings: GeneralSettings | null
  loadingGeneral: boolean
  errorGeneral: string | null
  fetchGeneralSettings: () => Promise<void>
  updateGeneralSettings: (language: string) => Promise<void>

  // Appearance Settings
  appearanceSettings: AppearanceSettings | null
  loadingAppearance: boolean
  errorAppearance: string | null
  fetchAppearanceSettings: () => Promise<void>
  updateAppearanceSettings: (theme: string) => Promise<void>

  // Database Settings
  databaseSettings: DatabaseSettings | null
  loadingDatabase: boolean
  errorDatabase: string | null
  fetchDatabaseSettings: () => Promise<void>
  updateDatabaseSettings: (databaseDirectory: string) => Promise<void>
}

export function useSettings(): UseSettingsReturn {
  // General Settings State
  const [generalSettings, setGeneralSettings] = useState<GeneralSettings | null>(null)
  const [loadingGeneral, setLoadingGeneral] = useState(false)
  const [errorGeneral, setErrorGeneral] = useState<string | null>(null)

  // Appearance Settings State
  const [appearanceSettings, setAppearanceSettings] = useState<AppearanceSettings | null>(null)
  const [loadingAppearance, setLoadingAppearance] = useState(false)
  const [errorAppearance, setErrorAppearance] = useState<string | null>(null)

  // Database Settings State
  const [databaseSettings, setDatabaseSettings] = useState<DatabaseSettings | null>(null)
  const [loadingDatabase, setLoadingDatabase] = useState(false)
  const [errorDatabase, setErrorDatabase] = useState<string | null>(null)

  // Fetch General Settings
  const fetchGeneralSettings = useCallback(async () => {
    setLoadingGeneral(true)
    setErrorGeneral(null)
    try {
      const result = await getGeneralSettings()
      if (result.errors) {
        setErrorGeneral(result.errors[0]?.message || "Failed to fetch general settings")
      } else if (result.data) {
        setGeneralSettings(result.data.generalSettings)
      }
    } catch (error) {
      setErrorGeneral(error instanceof Error ? error.message : "Unknown error")
    } finally {
      setLoadingGeneral(false)
    }
  }, [])

  // Update General Settings
  const updateGeneralSettings = useCallback(async (language: string) => {
    setLoadingGeneral(true)
    setErrorGeneral(null)
    try {
      const result = await updateGeneralSettingsApi(language)
      if (result.errors) {
        setErrorGeneral(result.errors[0]?.message || "Failed to update general settings")
      } else if (result.data) {
        setGeneralSettings(result.data.updateGeneralSettings)
      }
    } catch (error) {
      setErrorGeneral(error instanceof Error ? error.message : "Unknown error")
    } finally {
      setLoadingGeneral(false)
    }
  }, [])

  // Fetch Appearance Settings
  const fetchAppearanceSettings = useCallback(async () => {
    setLoadingAppearance(true)
    setErrorAppearance(null)
    try {
      const result = await getAppearanceSettings()
      if (result.errors) {
        setErrorAppearance(result.errors[0]?.message || "Failed to fetch appearance settings")
      } else if (result.data) {
        setAppearanceSettings(result.data.appearanceSettings)
      }
    } catch (error) {
      setErrorAppearance(error instanceof Error ? error.message : "Unknown error")
    } finally {
      setLoadingAppearance(false)
    }
  }, [])

  // Update Appearance Settings
  const updateAppearanceSettings = useCallback(async (theme: string) => {
    setLoadingAppearance(true)
    setErrorAppearance(null)
    try {
      const result = await updateAppearanceSettingsApi(theme)
      if (result.errors) {
        setErrorAppearance(result.errors[0]?.message || "Failed to update appearance settings")
      } else if (result.data) {
        setAppearanceSettings(result.data.updateAppearanceSettings)
      }
    } catch (error) {
      setErrorAppearance(error instanceof Error ? error.message : "Unknown error")
    } finally {
      setLoadingAppearance(false)
    }
  }, [])

  // Fetch Database Settings
  const fetchDatabaseSettings = useCallback(async () => {
    setLoadingDatabase(true)
    setErrorDatabase(null)
    try {
      const result = await getDatabaseSettings()
      if (result.errors) {
        setErrorDatabase(result.errors[0]?.message || "Failed to fetch database settings")
      } else if (result.data) {
        setDatabaseSettings(result.data.databaseSettings)
      }
    } catch (error) {
      setErrorDatabase(error instanceof Error ? error.message : "Unknown error")
    } finally {
      setLoadingDatabase(false)
    }
  }, [])

  // Update Database Settings
  const updateDatabaseSettings = useCallback(async (databaseDirectory: string) => {
    setLoadingDatabase(true)
    setErrorDatabase(null)
    try {
      const result = await updateDatabaseSettingsApi(databaseDirectory)
      if (result.errors) {
        setErrorDatabase(result.errors[0]?.message || "Failed to update database settings")
      } else if (result.data) {
        setDatabaseSettings(result.data.updateDatabaseSettings)
      }
    } catch (error) {
      setErrorDatabase(error instanceof Error ? error.message : "Unknown error")
    } finally {
      setLoadingDatabase(false)
    }
  }, [])

  return {
    generalSettings,
    loadingGeneral,
    errorGeneral,
    fetchGeneralSettings,
    updateGeneralSettings,

    appearanceSettings,
    loadingAppearance,
    errorAppearance,
    fetchAppearanceSettings,
    updateAppearanceSettings,

    databaseSettings,
    loadingDatabase,
    errorDatabase,
    fetchDatabaseSettings,
    updateDatabaseSettings,
  }
}
