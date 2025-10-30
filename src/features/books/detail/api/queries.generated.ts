// @ts-nocheck
import type * as Types from '../../../../generated/graphql';

import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type GetBookQueryVariables = Types.Exact<{
  id: Types.Scalars['Int']['input'];
}>;


export type GetBookQuery = { __typename?: 'QueryRoot', library: { __typename?: 'BookQuery', book?: { __typename?: 'BookDto', id: number, title: string, author?: string | null, description?: string | null, publishedYear?: number | null } | null } };


export const GetBookDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetBook"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"id"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"library"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"book"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"id"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"FragmentSpread","name":{"kind":"Name","value":"BookDetail"}}]}}]}}]}},{"kind":"FragmentDefinition","name":{"kind":"Name","value":"BookDetail"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"BookDto"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"publishedYear"}}]}}]} as unknown as DocumentNode<GetBookQuery, GetBookQueryVariables>;