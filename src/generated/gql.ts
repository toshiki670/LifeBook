/* eslint-disable */

import type { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core"
import * as types from "./graphql"

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
type Documents = {
  "mutation CreateBook($title: String!, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    createBook(\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation UpdateBook($id: Int!, $title: String, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    updateBook(\n      id: $id\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation DeleteBook($id: Int!) {\n  library {\n    deleteBook(id: $id)\n  }\n}": typeof types.CreateBookDocument
  "mutation UpdateGeneralSettings($language: String) {\n  settings {\n    updateGeneralSettings(language: $language) {\n      language\n    }\n  }\n}\n\nmutation UpdateAppearanceSettings($theme: String) {\n  settings {\n    updateAppearanceSettings(theme: $theme) {\n      theme\n    }\n  }\n}\n\nmutation UpdateDatabaseSettings($databaseDirectory: String) {\n  settings {\n    updateDatabaseSettings(databaseDirectory: $databaseDirectory) {\n      databaseDirectory\n    }\n  }\n}\n\nmutation ResetSettings {\n  settings {\n    resetSettings\n  }\n}": typeof types.UpdateGeneralSettingsDocument
  "query GetBooks {\n  library {\n    books {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nquery GetBook($id: Int!) {\n  library {\n    book(id: $id) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}": typeof types.GetBooksDocument
  "query GetGeneralSettings {\n  settings {\n    generalSettings {\n      language\n    }\n  }\n}\n\nquery GetAppearanceSettings {\n  settings {\n    appearanceSettings {\n      theme\n    }\n  }\n}\n\nquery GetDatabaseSettings {\n  settings {\n    databaseSettings {\n      databaseDirectory\n    }\n  }\n}": typeof types.GetGeneralSettingsDocument
}
const documents: Documents = {
  "mutation CreateBook($title: String!, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    createBook(\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation UpdateBook($id: Int!, $title: String, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    updateBook(\n      id: $id\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation DeleteBook($id: Int!) {\n  library {\n    deleteBook(id: $id)\n  }\n}":
    types.CreateBookDocument,
  "mutation UpdateGeneralSettings($language: String) {\n  settings {\n    updateGeneralSettings(language: $language) {\n      language\n    }\n  }\n}\n\nmutation UpdateAppearanceSettings($theme: String) {\n  settings {\n    updateAppearanceSettings(theme: $theme) {\n      theme\n    }\n  }\n}\n\nmutation UpdateDatabaseSettings($databaseDirectory: String) {\n  settings {\n    updateDatabaseSettings(databaseDirectory: $databaseDirectory) {\n      databaseDirectory\n    }\n  }\n}\n\nmutation ResetSettings {\n  settings {\n    resetSettings\n  }\n}":
    types.UpdateGeneralSettingsDocument,
  "query GetBooks {\n  library {\n    books {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nquery GetBook($id: Int!) {\n  library {\n    book(id: $id) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}":
    types.GetBooksDocument,
  "query GetGeneralSettings {\n  settings {\n    generalSettings {\n      language\n    }\n  }\n}\n\nquery GetAppearanceSettings {\n  settings {\n    appearanceSettings {\n      theme\n    }\n  }\n}\n\nquery GetDatabaseSettings {\n  settings {\n    databaseSettings {\n      databaseDirectory\n    }\n  }\n}":
    types.GetGeneralSettingsDocument,
}

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "mutation CreateBook($title: String!, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    createBook(\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation UpdateBook($id: Int!, $title: String, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    updateBook(\n      id: $id\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation DeleteBook($id: Int!) {\n  library {\n    deleteBook(id: $id)\n  }\n}",
): (typeof documents)["mutation CreateBook($title: String!, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    createBook(\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation UpdateBook($id: Int!, $title: String, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    updateBook(\n      id: $id\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation DeleteBook($id: Int!) {\n  library {\n    deleteBook(id: $id)\n  }\n}"]
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "mutation UpdateGeneralSettings($language: String) {\n  settings {\n    updateGeneralSettings(language: $language) {\n      language\n    }\n  }\n}\n\nmutation UpdateAppearanceSettings($theme: String) {\n  settings {\n    updateAppearanceSettings(theme: $theme) {\n      theme\n    }\n  }\n}\n\nmutation UpdateDatabaseSettings($databaseDirectory: String) {\n  settings {\n    updateDatabaseSettings(databaseDirectory: $databaseDirectory) {\n      databaseDirectory\n    }\n  }\n}\n\nmutation ResetSettings {\n  settings {\n    resetSettings\n  }\n}",
): (typeof documents)["mutation UpdateGeneralSettings($language: String) {\n  settings {\n    updateGeneralSettings(language: $language) {\n      language\n    }\n  }\n}\n\nmutation UpdateAppearanceSettings($theme: String) {\n  settings {\n    updateAppearanceSettings(theme: $theme) {\n      theme\n    }\n  }\n}\n\nmutation UpdateDatabaseSettings($databaseDirectory: String) {\n  settings {\n    updateDatabaseSettings(databaseDirectory: $databaseDirectory) {\n      databaseDirectory\n    }\n  }\n}\n\nmutation ResetSettings {\n  settings {\n    resetSettings\n  }\n}"]
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "query GetBooks {\n  library {\n    books {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nquery GetBook($id: Int!) {\n  library {\n    book(id: $id) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}",
): (typeof documents)["query GetBooks {\n  library {\n    books {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nquery GetBook($id: Int!) {\n  library {\n    book(id: $id) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}"]
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(
  source: "query GetGeneralSettings {\n  settings {\n    generalSettings {\n      language\n    }\n  }\n}\n\nquery GetAppearanceSettings {\n  settings {\n    appearanceSettings {\n      theme\n    }\n  }\n}\n\nquery GetDatabaseSettings {\n  settings {\n    databaseSettings {\n      databaseDirectory\n    }\n  }\n}",
): (typeof documents)["query GetGeneralSettings {\n  settings {\n    generalSettings {\n      language\n    }\n  }\n}\n\nquery GetAppearanceSettings {\n  settings {\n    appearanceSettings {\n      theme\n    }\n  }\n}\n\nquery GetDatabaseSettings {\n  settings {\n    databaseSettings {\n      databaseDirectory\n    }\n  }\n}"]

export function graphql(source: string) {
  return (documents as any)[source] ?? {}
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> =
  TDocumentNode extends DocumentNode<infer TType, any> ? TType : never
