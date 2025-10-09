import { invoke } from "@tauri-apps/api/core"

export interface GraphQLResponse<T = unknown> {
  data?: T
  errors?: Array<{
    message: string
    locations?: Array<{ line: number; column: number }>
    path?: string[]
  }>
}

/**
 * GraphQLクエリを実行
 */
export async function executeGraphQL<T = unknown>(query: string): Promise<GraphQLResponse<T>> {
  try {
    const result = await invoke<string>("execute_graphql", { query })
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
      books {
        id
        title
        author
        description
        publishedYear
      }
    }
  `
  return executeGraphQL(query)
}

/**
 * IDで本を取得
 */
export async function getBook(id: number) {
  const query = `
    query {
      book(id: ${id}) {
        id
        title
        author
        description
        publishedYear
      }
    }
  `
  return executeGraphQL(query)
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
    mutation {
      createBook(
        title: "${book.title}"
        ${book.author ? `author: "${book.author}"` : ""}
        ${book.description ? `description: "${book.description}"` : ""}
        ${book.publishedYear ? `publishedYear: ${book.publishedYear}` : ""}
      ) {
        id
        title
        author
        description
        publishedYear
      }
    }
  `
  return executeGraphQL(query)
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
  const fields = []
  if (updates.title) fields.push(`title: "${updates.title}"`)
  if (updates.author) fields.push(`author: "${updates.author}"`)
  if (updates.description) fields.push(`description: "${updates.description}"`)
  if (updates.publishedYear) fields.push(`publishedYear: ${updates.publishedYear}`)

  const query = `
    mutation {
      updateBook(id: ${id}, ${fields.join(", ")}) {
        id
        title
        author
        description
        publishedYear
      }
    }
  `
  return executeGraphQL(query)
}

/**
 * 本を削除
 */
export async function deleteBook(id: number) {
  const query = `
    mutation {
      deleteBook(id: ${id})
    }
  `
  return executeGraphQL(query)
}

/**
 * データベース接続状態を確認
 */
export async function getDbStatus(): Promise<string> {
  return invoke<string>("get_db_status")
}
