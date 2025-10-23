// Settings Feature - Type Definitions

export type SettingsSection = "general" | "appearance" | "database"

export interface GeneralSettings {
  language: string
}

export interface AppearanceSettings {
  theme: string
}

export interface DatabaseSettings {
  databaseDirectory: string
}

export interface SettingsSectionItem {
  id: SettingsSection
  label: string
  icon?: React.ComponentType<{ className?: string }>
}
