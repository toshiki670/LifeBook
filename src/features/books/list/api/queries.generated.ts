// @ts-nocheck
import type * as Types from '../../../../generated/graphql';

import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type GetAllBooksQueryVariables = Types.Exact<{ [key: string]: never; }>;


export type GetAllBooksQuery = { __typename?: 'QueryRoot', library: { __typename?: 'BookQuery', books: Array<{ __typename?: 'BookDto', id: number, title: string, author?: string | null }> } };


export const GetAllBooksDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetAllBooks"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"library"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"books"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"BookCard"}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"BookCard"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"BookDto"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}}]}}]} as unknown as DocumentNode<GetAllBooksQuery, GetAllBooksQueryVariables>;