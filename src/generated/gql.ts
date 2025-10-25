/* eslint-disable */
import * as types from './graphql';
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

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
    "mutation BooksCreate($title: String!, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    createBook(\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation BooksUpdate($id: Int!, $title: String, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    updateBook(\n      id: $id\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation BooksDelete($id: Int!) {\n  library {\n    deleteBook(id: $id)\n  }\n}": typeof types.BooksCreateDocument,
    "query BooksGetAll {\n  library {\n    books {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nquery BooksGetOne($id: Int!) {\n  library {\n    book(id: $id) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}": typeof types.BooksGetAllDocument,
    "mutation SettingsUpdateGeneral($language: String!) {\n  settings {\n    updateGeneralSettings(language: $language) {\n      language\n    }\n  }\n}\n\nmutation SettingsUpdateAppearance($theme: String!) {\n  settings {\n    updateAppearanceSettings(theme: $theme) {\n      theme\n    }\n  }\n}\n\nmutation SettingsUpdateDatabase($databaseDirectory: String!) {\n  settings {\n    updateDatabaseSettings(databaseDirectory: $databaseDirectory) {\n      databaseDirectory\n    }\n  }\n}": typeof types.SettingsUpdateGeneralDocument,
    "query SettingsGetGeneral {\n  settings {\n    generalSettings {\n      language\n    }\n  }\n}\n\nquery SettingsGetAppearance {\n  settings {\n    appearanceSettings {\n      theme\n    }\n  }\n}\n\nquery SettingsGetDatabase {\n  settings {\n    databaseSettings {\n      databaseDirectory\n    }\n  }\n}": typeof types.SettingsGetGeneralDocument,
};
const documents: Documents = {
    "mutation BooksCreate($title: String!, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    createBook(\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation BooksUpdate($id: Int!, $title: String, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    updateBook(\n      id: $id\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation BooksDelete($id: Int!) {\n  library {\n    deleteBook(id: $id)\n  }\n}": types.BooksCreateDocument,
    "query BooksGetAll {\n  library {\n    books {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nquery BooksGetOne($id: Int!) {\n  library {\n    book(id: $id) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}": types.BooksGetAllDocument,
    "mutation SettingsUpdateGeneral($language: String!) {\n  settings {\n    updateGeneralSettings(language: $language) {\n      language\n    }\n  }\n}\n\nmutation SettingsUpdateAppearance($theme: String!) {\n  settings {\n    updateAppearanceSettings(theme: $theme) {\n      theme\n    }\n  }\n}\n\nmutation SettingsUpdateDatabase($databaseDirectory: String!) {\n  settings {\n    updateDatabaseSettings(databaseDirectory: $databaseDirectory) {\n      databaseDirectory\n    }\n  }\n}": types.SettingsUpdateGeneralDocument,
    "query SettingsGetGeneral {\n  settings {\n    generalSettings {\n      language\n    }\n  }\n}\n\nquery SettingsGetAppearance {\n  settings {\n    appearanceSettings {\n      theme\n    }\n  }\n}\n\nquery SettingsGetDatabase {\n  settings {\n    databaseSettings {\n      databaseDirectory\n    }\n  }\n}": types.SettingsGetGeneralDocument,
};

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
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation BooksCreate($title: String!, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    createBook(\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation BooksUpdate($id: Int!, $title: String, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    updateBook(\n      id: $id\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation BooksDelete($id: Int!) {\n  library {\n    deleteBook(id: $id)\n  }\n}"): (typeof documents)["mutation BooksCreate($title: String!, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    createBook(\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation BooksUpdate($id: Int!, $title: String, $author: String, $description: String, $publishedYear: Int) {\n  library {\n    updateBook(\n      id: $id\n      title: $title\n      author: $author\n      description: $description\n      publishedYear: $publishedYear\n    ) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nmutation BooksDelete($id: Int!) {\n  library {\n    deleteBook(id: $id)\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query BooksGetAll {\n  library {\n    books {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nquery BooksGetOne($id: Int!) {\n  library {\n    book(id: $id) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}"): (typeof documents)["query BooksGetAll {\n  library {\n    books {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}\n\nquery BooksGetOne($id: Int!) {\n  library {\n    book(id: $id) {\n      id\n      title\n      author\n      description\n      publishedYear\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "mutation SettingsUpdateGeneral($language: String!) {\n  settings {\n    updateGeneralSettings(language: $language) {\n      language\n    }\n  }\n}\n\nmutation SettingsUpdateAppearance($theme: String!) {\n  settings {\n    updateAppearanceSettings(theme: $theme) {\n      theme\n    }\n  }\n}\n\nmutation SettingsUpdateDatabase($databaseDirectory: String!) {\n  settings {\n    updateDatabaseSettings(databaseDirectory: $databaseDirectory) {\n      databaseDirectory\n    }\n  }\n}"): (typeof documents)["mutation SettingsUpdateGeneral($language: String!) {\n  settings {\n    updateGeneralSettings(language: $language) {\n      language\n    }\n  }\n}\n\nmutation SettingsUpdateAppearance($theme: String!) {\n  settings {\n    updateAppearanceSettings(theme: $theme) {\n      theme\n    }\n  }\n}\n\nmutation SettingsUpdateDatabase($databaseDirectory: String!) {\n  settings {\n    updateDatabaseSettings(databaseDirectory: $databaseDirectory) {\n      databaseDirectory\n    }\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query SettingsGetGeneral {\n  settings {\n    generalSettings {\n      language\n    }\n  }\n}\n\nquery SettingsGetAppearance {\n  settings {\n    appearanceSettings {\n      theme\n    }\n  }\n}\n\nquery SettingsGetDatabase {\n  settings {\n    databaseSettings {\n      databaseDirectory\n    }\n  }\n}"): (typeof documents)["query SettingsGetGeneral {\n  settings {\n    generalSettings {\n      language\n    }\n  }\n}\n\nquery SettingsGetAppearance {\n  settings {\n    appearanceSettings {\n      theme\n    }\n  }\n}\n\nquery SettingsGetDatabase {\n  settings {\n    databaseSettings {\n      databaseDirectory\n    }\n  }\n}"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;