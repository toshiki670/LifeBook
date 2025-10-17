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
