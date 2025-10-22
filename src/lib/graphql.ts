import { invoke } from "@tauri-apps/api/core"

export interface GraphQLResponse<T = unknown> {
  data?: T
  errors?: Array<{
    message: string
    locations?: Array<{ line: number; column: number }>
    path?: string[]
  }>
}

export interface Book {
  id: number
  title: string
  author?: string
  description?: string
  publishedYear?: number
}

/**
 * GraphQLクエリを実行
 */
export async function executeGraphQL<T = unknown>(
  query: string,
  variables?: Record<string, unknown>,
): Promise<GraphQLResponse<T>> {
  try {
    const request = {
      query,
      ...(variables && { variables }),
    }
    const result = await invoke<string>("graphql_request", { request: JSON.stringify(request) })
    return JSON.parse(result)
  } catch (error) {
    console.error("GraphQL execution error:", error)
    throw error
  }
}

/**
 * すべての本を取得
 */
export async function getBooks() {
  const query = `
    query {
      library {
        books {
          id
          title
          author
          description
          publishedYear
        }
      }
    }
  `
  const result = await executeGraphQL<{ library: { books: Book[] } }>(query)
  return {
    data: result.data ? { books: result.data.library.books } : undefined,
    errors: result.errors,
  }
}

/**
 * IDで本を取得
 */
export async function getBook(id: number) {
  const query = `
    query {
      library {
        book(id: ${id}) {
          id
          title
          author
          description
          publishedYear
        }
      }
    }
  `
  const result = await executeGraphQL<{ library: { book: Book } }>(query)
  return {
    data: result.data ? { book: result.data.library.book } : undefined,
    errors: result.errors,
  }
}

/**
 * 新しい本を作成
 */
export async function createBook(book: {
  title: string
  author?: string
  description?: string
  publishedYear?: number
}) {
  const query = `
    mutation CreateBook($title: String!, $author: String, $description: String, $publishedYear: Int) {
      library {
        createBook(
          title: $title
          author: $author
          description: $description
          publishedYear: $publishedYear
        ) {
          id
          title
          author
          description
          publishedYear
        }
      }
    }
  `
  const variables = {
    title: book.title,
    author: book.author || null,
    description: book.description || null,
    publishedYear: book.publishedYear || null,
  }

  const result = await executeGraphQL<{ library: { createBook: Book } }>(query, variables)
  return {
    data: result.data ? { createBook: result.data.library.createBook } : undefined,
    errors: result.errors,
  }
}

/**
 * 本を更新
 */
export async function updateBook(
  id: number,
  updates: {
    title?: string
    author?: string
    description?: string
    publishedYear?: number
  },
) {
  const query = `
    mutation UpdateBook($id: Int!, $title: String, $author: String, $description: String, $publishedYear: Int) {
      library {
        updateBook(
          id: $id
          title: $title
          author: $author
          description: $description
          publishedYear: $publishedYear
        ) {
          id
          title
          author
          description
          publishedYear
        }
      }
    }
  `
  const variables = {
    id,
    title: updates.title || null,
    author: updates.author || null,
    description: updates.description || null,
    publishedYear: updates.publishedYear || null,
  }

  const result = await executeGraphQL<{ library: { updateBook: Book } }>(query, variables)
  return {
    data: result.data ? { updateBook: result.data.library.updateBook } : undefined,
    errors: result.errors,
  }
}

/**
 * 本を削除
 */
export async function deleteBook(id: number) {
  const query = `
    mutation DeleteBook($id: Int!) {
      library {
        deleteBook(id: $id)
      }
    }
  `
  const variables = { id }

  const result = await executeGraphQL<{ library: { deleteBook: boolean } }>(query, variables)
  return {
    data: result.data ? { deleteBook: result.data.library.deleteBook } : undefined,
    errors: result.errors,
  }
}

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
