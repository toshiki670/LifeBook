export type Maybe<T> = T | null
export type InputMaybe<T> = Maybe<T>
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] }
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> }
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> }
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = {
  [_ in K]?: never
}
export type Incremental<T> =
  | T
  | { [P in keyof T]?: P extends " $fragmentName" | "__typename" ? T[P] : never }
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string }
  String: { input: string; output: string }
  Boolean: { input: boolean; output: boolean }
  Int: { input: number; output: number }
  Float: { input: number; output: number }
}

/** 表示設定のDTO */
export type AppearanceSettingsDto = {
  __typename?: "AppearanceSettingsDto"
  theme: Scalars["String"]["output"]
}

/** Book DTO - GraphQLレスポンス用 */
export type BookDto = {
  __typename?: "BookDto"
  author?: Maybe<Scalars["String"]["output"]>
  description?: Maybe<Scalars["String"]["output"]>
  id: Scalars["Int"]["output"]
  publishedYear?: Maybe<Scalars["Int"]["output"]>
  title: Scalars["String"]["output"]
}

export type BookMutation = {
  __typename?: "BookMutation"
  /** 新しい本を作成 */
  createBook: BookDto
  /** 本を削除 */
  deleteBook: Scalars["Boolean"]["output"]
  /** 本を更新 */
  updateBook: BookDto
}

export type BookMutationCreateBookArgs = {
  author?: InputMaybe<Scalars["String"]["input"]>
  description?: InputMaybe<Scalars["String"]["input"]>
  publishedYear?: InputMaybe<Scalars["Int"]["input"]>
  title: Scalars["String"]["input"]
}

export type BookMutationDeleteBookArgs = {
  id: Scalars["Int"]["input"]
}

export type BookMutationUpdateBookArgs = {
  author?: InputMaybe<Scalars["String"]["input"]>
  description?: InputMaybe<Scalars["String"]["input"]>
  id: Scalars["Int"]["input"]
  publishedYear?: InputMaybe<Scalars["Int"]["input"]>
  title?: InputMaybe<Scalars["String"]["input"]>
}

export type BookQuery = {
  __typename?: "BookQuery"
  /** IDで本を取得 */
  book?: Maybe<BookDto>
  /** すべての本を取得 */
  books: Array<BookDto>
}

export type BookQueryBookArgs = {
  id: Scalars["Int"]["input"]
}

/** データベース設定のDTO */
export type DatabaseSettingsDto = {
  __typename?: "DatabaseSettingsDto"
  databaseDirectory: Scalars["String"]["output"]
}

/** 一般設定のDTO */
export type GeneralSettingsDto = {
  __typename?: "GeneralSettingsDto"
  language: Scalars["String"]["output"]
}

export type MutationRoot = {
  __typename?: "MutationRoot"
  /** Libraryコンテキストのミューテーション */
  library: BookMutation
  /** Settingsコンテキストのミューテーション */
  settings: SettingsMutation
}

export type QueryRoot = {
  __typename?: "QueryRoot"
  /** Libraryコンテキストへのアクセス */
  library: BookQuery
  /** Settingsコンテキストへのアクセス */
  settings: SettingsQuery
}

export type SettingsMutation = {
  __typename?: "SettingsMutation"
  /** すべての設定をリセット */
  resetSettings: Scalars["Boolean"]["output"]
  /** 表示設定を更新 */
  updateAppearanceSettings: AppearanceSettingsDto
  /** データベース設定を更新 */
  updateDatabaseSettings: DatabaseSettingsDto
  /** 一般設定を更新 */
  updateGeneralSettings: GeneralSettingsDto
}

export type SettingsMutationUpdateAppearanceSettingsArgs = {
  theme?: InputMaybe<Scalars["String"]["input"]>
}

export type SettingsMutationUpdateDatabaseSettingsArgs = {
  databaseDirectory?: InputMaybe<Scalars["String"]["input"]>
}

export type SettingsMutationUpdateGeneralSettingsArgs = {
  language?: InputMaybe<Scalars["String"]["input"]>
}

export type SettingsQuery = {
  __typename?: "SettingsQuery"
  /** 表示設定を取得 */
  appearanceSettings: AppearanceSettingsDto
  /** データベース設定を取得 */
  databaseSettings: DatabaseSettingsDto
  /** 一般設定を取得 */
  generalSettings: GeneralSettingsDto
}
