import { ApolloClient, ApolloLink, InMemoryCache, Observable } from "@apollo/client"
import { invoke } from "@tauri-apps/api/core"
import { print } from "graphql"

/**
 * Tauri invoke経由でGraphQLリクエストを実行するカスタムLink
 */
const tauriLink = new ApolloLink((operation) => {
  return new Observable((observer) => {
    const { query, variables, operationName } = operation

    // クエリ文字列の取得
    // TypedDocumentString (string mode) または DocumentNode (documentNode mode) をサポート
    let queryString = ""
    try {
      if (typeof query === "string") {
        // 文字列形式のクエリ
        queryString = query
      } else if (query.loc?.source?.body) {
        // DocumentNode形式のクエリ (loc.source.bodyがある場合)
        queryString = query.loc.source.body
      } else {
        // DocumentNode形式のクエリ (loc.source.bodyがない場合、printを使用)
        queryString = print(query)
      }
    } catch (error) {
      observer.error(error)
      return
    }

    if (!queryString.trim()) {
      observer.error(new Error("Empty GraphQL query string"))
      return
    }

    // GraphQLリクエストを構築
    const request = {
      query: queryString,
      variables,
      operationName,
    }

    // Tauri invoke経由でGraphQLリクエストを実行
    invoke<string>("graphql_request", { request: JSON.stringify(request) })
      .then((result) => {
        try {
          const response = JSON.parse(result)
          observer.next(response)
          observer.complete()
        } catch (parseError) {
          observer.error(parseError)
        }
      })
      .catch((error) => {
        observer.error(error)
      })
  })
})

/**
 * Apollo Clientのインスタンス
 */
export const apolloClient = new ApolloClient({
  link: tauriLink,
  cache: new InMemoryCache(),
  defaultOptions: {
    watchQuery: {
      fetchPolicy: "cache-and-network",
      errorPolicy: "all",
    },
    query: {
      fetchPolicy: "network-only",
      errorPolicy: "all",
    },
    mutate: {
      errorPolicy: "all",
    },
  },
})
