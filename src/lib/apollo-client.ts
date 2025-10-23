import { ApolloClient, ApolloLink, InMemoryCache, Observable } from "@apollo/client"
import { invoke } from "@tauri-apps/api/core"

/**
 * Tauri invoke経由でGraphQLリクエストを実行するカスタムLink
 */
const tauriLink = new ApolloLink((operation) => {
  return new Observable((observer) => {
    const { query, variables, operationName } = operation

    // GraphQLリクエストを構築
    const request = {
      query: query.loc?.source.body || "",
      variables,
      operationName,
    }

    // Tauri invoke経由でGraphQLリクエストを実行
    invoke<string>("graphql_request", { request: JSON.stringify(request) })
      .then((result) => {
        const response = JSON.parse(result)
        observer.next(response)
        observer.complete()
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
