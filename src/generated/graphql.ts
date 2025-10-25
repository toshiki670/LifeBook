/* eslint-disable */
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = T | null | undefined;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

/** 表示設定のDTO */
export type AppearanceSettingsDto = {
  __typename?: 'AppearanceSettingsDto';
  theme: Scalars['String']['output'];
};

/** Book DTO - GraphQLレスポンス用 */
export type BookDto = {
  __typename?: 'BookDto';
  author?: Maybe<Scalars['String']['output']>;
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  publishedYear?: Maybe<Scalars['Int']['output']>;
  title: Scalars['String']['output'];
};

export type BookMutation = {
  __typename?: 'BookMutation';
  /** 新しい本を作成 */
  createBook: BookDto;
  /** 本を削除 */
  deleteBook: Scalars['Boolean']['output'];
  /** 本を更新 */
  updateBook: BookDto;
};


export type BookMutationCreateBookArgs = {
  author?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  publishedYear?: InputMaybe<Scalars['Int']['input']>;
  title: Scalars['String']['input'];
};


export type BookMutationDeleteBookArgs = {
  id: Scalars['Int']['input'];
};


export type BookMutationUpdateBookArgs = {
  author?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  publishedYear?: InputMaybe<Scalars['Int']['input']>;
  title?: InputMaybe<Scalars['String']['input']>;
};

export type BookQuery = {
  __typename?: 'BookQuery';
  /** IDで本を取得 */
  book?: Maybe<BookDto>;
  /** すべての本を取得 */
  books: Array<BookDto>;
};


export type BookQueryBookArgs = {
  id: Scalars['Int']['input'];
};

/** データベース設定のDTO */
export type DatabaseSettingsDto = {
  __typename?: 'DatabaseSettingsDto';
  databaseDirectory: Scalars['String']['output'];
};

/** 一般設定のDTO */
export type GeneralSettingsDto = {
  __typename?: 'GeneralSettingsDto';
  language: Scalars['String']['output'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  /** Libraryコンテキストのミューテーション */
  library: BookMutation;
  /** Settingsコンテキストのミューテーション */
  settings: SettingsMutation;
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  /** Libraryコンテキストへのアクセス */
  library: BookQuery;
  /** Settingsコンテキストへのアクセス */
  settings: SettingsQuery;
};

export type SettingsMutation = {
  __typename?: 'SettingsMutation';
  /** すべての設定をリセット */
  resetSettings: Scalars['Boolean']['output'];
  /** 表示設定を更新 */
  updateAppearanceSettings: AppearanceSettingsDto;
  /** データベース設定を更新 */
  updateDatabaseSettings: DatabaseSettingsDto;
  /** 一般設定を更新 */
  updateGeneralSettings: GeneralSettingsDto;
};


export type SettingsMutationUpdateAppearanceSettingsArgs = {
  theme?: InputMaybe<Scalars['String']['input']>;
};


export type SettingsMutationUpdateDatabaseSettingsArgs = {
  databaseDirectory?: InputMaybe<Scalars['String']['input']>;
};


export type SettingsMutationUpdateGeneralSettingsArgs = {
  language?: InputMaybe<Scalars['String']['input']>;
};

export type SettingsQuery = {
  __typename?: 'SettingsQuery';
  /** 表示設定を取得 */
  appearanceSettings: AppearanceSettingsDto;
  /** データベース設定を取得 */
  databaseSettings: DatabaseSettingsDto;
  /** 一般設定を取得 */
  generalSettings: GeneralSettingsDto;
};

export type BooksCreateMutationVariables = Exact<{
  title: Scalars['String']['input'];
  author?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  publishedYear?: InputMaybe<Scalars['Int']['input']>;
}>;


export type BooksCreateMutation = { __typename?: 'MutationRoot', library: { __typename?: 'BookMutation', createBook: { __typename?: 'BookDto', id: number, title: string, author?: string | null, description?: string | null, publishedYear?: number | null } } };

export type BooksUpdateMutationVariables = Exact<{
  id: Scalars['Int']['input'];
  title?: InputMaybe<Scalars['String']['input']>;
  author?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  publishedYear?: InputMaybe<Scalars['Int']['input']>;
}>;


export type BooksUpdateMutation = { __typename?: 'MutationRoot', library: { __typename?: 'BookMutation', updateBook: { __typename?: 'BookDto', id: number, title: string, author?: string | null, description?: string | null, publishedYear?: number | null } } };

export type BooksDeleteMutationVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type BooksDeleteMutation = { __typename?: 'MutationRoot', library: { __typename?: 'BookMutation', deleteBook: boolean } };

export type BooksGetAllQueryVariables = Exact<{ [key: string]: never; }>;


export type BooksGetAllQuery = { __typename?: 'QueryRoot', library: { __typename?: 'BookQuery', books: Array<{ __typename?: 'BookDto', id: number, title: string, author?: string | null, description?: string | null, publishedYear?: number | null }> } };

export type BooksGetOneQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type BooksGetOneQuery = { __typename?: 'QueryRoot', library: { __typename?: 'BookQuery', book?: { __typename?: 'BookDto', id: number, title: string, author?: string | null, description?: string | null, publishedYear?: number | null } | null } };

export type SettingsUpdateGeneralMutationVariables = Exact<{
  language: Scalars['String']['input'];
}>;


export type SettingsUpdateGeneralMutation = { __typename?: 'MutationRoot', settings: { __typename?: 'SettingsMutation', updateGeneralSettings: { __typename?: 'GeneralSettingsDto', language: string } } };

export type SettingsUpdateAppearanceMutationVariables = Exact<{
  theme: Scalars['String']['input'];
}>;


export type SettingsUpdateAppearanceMutation = { __typename?: 'MutationRoot', settings: { __typename?: 'SettingsMutation', updateAppearanceSettings: { __typename?: 'AppearanceSettingsDto', theme: string } } };

export type SettingsUpdateDatabaseMutationVariables = Exact<{
  databaseDirectory: Scalars['String']['input'];
}>;


export type SettingsUpdateDatabaseMutation = { __typename?: 'MutationRoot', settings: { __typename?: 'SettingsMutation', updateDatabaseSettings: { __typename?: 'DatabaseSettingsDto', databaseDirectory: string } } };

export type SettingsGetGeneralQueryVariables = Exact<{ [key: string]: never; }>;


export type SettingsGetGeneralQuery = { __typename?: 'QueryRoot', settings: { __typename?: 'SettingsQuery', generalSettings: { __typename?: 'GeneralSettingsDto', language: string } } };

export type SettingsGetAppearanceQueryVariables = Exact<{ [key: string]: never; }>;


export type SettingsGetAppearanceQuery = { __typename?: 'QueryRoot', settings: { __typename?: 'SettingsQuery', appearanceSettings: { __typename?: 'AppearanceSettingsDto', theme: string } } };

export type SettingsGetDatabaseQueryVariables = Exact<{ [key: string]: never; }>;


export type SettingsGetDatabaseQuery = { __typename?: 'QueryRoot', settings: { __typename?: 'SettingsQuery', databaseSettings: { __typename?: 'DatabaseSettingsDto', databaseDirectory: string } } };


export const BooksCreateDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"BooksCreate"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"title"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"author"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"description"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"publishedYear"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"library"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"createBook"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"title"},"value":{"kind":"Variable","name":{"kind":"Name","value":"title"}}},{"kind":"Argument","name":{"kind":"Name","value":"author"},"value":{"kind":"Variable","name":{"kind":"Name","value":"author"}}},{"kind":"Argument","name":{"kind":"Name","value":"description"},"value":{"kind":"Variable","name":{"kind":"Name","value":"description"}}},{"kind":"Argument","name":{"kind":"Name","value":"publishedYear"},"value":{"kind":"Variable","name":{"kind":"Name","value":"publishedYear"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"publishedYear"}}]}}]}}]}}]} as unknown as DocumentNode<BooksCreateMutation, BooksCreateMutationVariables>;
export const BooksUpdateDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"BooksUpdate"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"title"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"author"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"description"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"publishedYear"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"library"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateBook"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}},{"kind":"Argument","name":{"kind":"Name","value":"title"},"value":{"kind":"Variable","name":{"kind":"Name","value":"title"}}},{"kind":"Argument","name":{"kind":"Name","value":"author"},"value":{"kind":"Variable","name":{"kind":"Name","value":"author"}}},{"kind":"Argument","name":{"kind":"Name","value":"description"},"value":{"kind":"Variable","name":{"kind":"Name","value":"description"}}},{"kind":"Argument","name":{"kind":"Name","value":"publishedYear"},"value":{"kind":"Variable","name":{"kind":"Name","value":"publishedYear"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"publishedYear"}}]}}]}}]}}]} as unknown as DocumentNode<BooksUpdateMutation, BooksUpdateMutationVariables>;
export const BooksDeleteDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"BooksDelete"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"library"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"deleteBook"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}}]}]}}]}}]} as unknown as DocumentNode<BooksDeleteMutation, BooksDeleteMutationVariables>;
export const BooksGetAllDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"BooksGetAll"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"library"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"books"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"publishedYear"}}]}}]}}]}}]} as unknown as DocumentNode<BooksGetAllQuery, BooksGetAllQueryVariables>;
export const BooksGetOneDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"BooksGetOne"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"library"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"book"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"publishedYear"}}]}}]}}]}}]} as unknown as DocumentNode<BooksGetOneQuery, BooksGetOneQueryVariables>;
export const SettingsUpdateGeneralDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"SettingsUpdateGeneral"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"language"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateGeneralSettings"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"language"},"value":{"kind":"Variable","name":{"kind":"Name","value":"language"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"language"}}]}}]}}]}}]} as unknown as DocumentNode<SettingsUpdateGeneralMutation, SettingsUpdateGeneralMutationVariables>;
export const SettingsUpdateAppearanceDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"SettingsUpdateAppearance"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"theme"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateAppearanceSettings"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"theme"},"value":{"kind":"Variable","name":{"kind":"Name","value":"theme"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"theme"}}]}}]}}]}}]} as unknown as DocumentNode<SettingsUpdateAppearanceMutation, SettingsUpdateAppearanceMutationVariables>;
export const SettingsUpdateDatabaseDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"SettingsUpdateDatabase"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"databaseDirectory"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateDatabaseSettings"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"databaseDirectory"},"value":{"kind":"Variable","name":{"kind":"Name","value":"databaseDirectory"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"databaseDirectory"}}]}}]}}]}}]} as unknown as DocumentNode<SettingsUpdateDatabaseMutation, SettingsUpdateDatabaseMutationVariables>;
export const SettingsGetGeneralDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"SettingsGetGeneral"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"generalSettings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"language"}}]}}]}}]}}]} as unknown as DocumentNode<SettingsGetGeneralQuery, SettingsGetGeneralQueryVariables>;
export const SettingsGetAppearanceDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"SettingsGetAppearance"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"appearanceSettings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"theme"}}]}}]}}]}}]} as unknown as DocumentNode<SettingsGetAppearanceQuery, SettingsGetAppearanceQueryVariables>;
export const SettingsGetDatabaseDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"SettingsGetDatabase"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"databaseSettings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"databaseDirectory"}}]}}]}}]}}]} as unknown as DocumentNode<SettingsGetDatabaseQuery, SettingsGetDatabaseQueryVariables>;