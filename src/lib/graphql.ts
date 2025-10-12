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
  return executeGraphQL<{ books: Book[] }>(query)
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
  return executeGraphQL<{ book: Book }>(query)
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
  `
  const variables = {
    title: book.title,
    author: book.author || null,
    description: book.description || null,
    publishedYear: book.publishedYear || null,
  }
  
  const request = {
    query,
    variables,
  }
  
  const result = await invoke<string>("graphql_request", { request: JSON.stringify(request) })
  return JSON.parse(result) as GraphQLResponse<{ createBook: Book }>
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
  `
  const variables = {
    id,
    title: updates.title || null,
    author: updates.author || null,
    description: updates.description || null,
    publishedYear: updates.publishedYear || null,
  }
  
  const request = {
    query,
    variables,
  }
  
  const result = await invoke<string>("graphql_request", { request: JSON.stringify(request) })
  return JSON.parse(result) as GraphQLResponse<{ updateBook: Book }>
}

/**
 * 本を削除
 */
export async function deleteBook(id: number) {
  const query = `
    mutation DeleteBook($id: Int!) {
      deleteBook(id: $id)
    }
  `
  const variables = { id }
  
  const request = {
    query,
    variables,
  }
  
  const result = await invoke<string>("graphql_request", { request: JSON.stringify(request) })
  return JSON.parse(result) as GraphQLResponse<{ deleteBook: boolean }>
}

/**
 * データベース接続状態を確認
 */
export async function getDbStatus(): Promise<string> {
  return invoke<string>("get_db_status")
}
