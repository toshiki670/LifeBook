// @ts-nocheck
import type * as Types from '../../../../generated/graphql';

import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type GetDatabaseSettingsQueryVariables = Types.Exact<{ [key: string]: never; }>;


export type GetDatabaseSettingsQuery = { __typename?: 'QueryRoot', settings: { __typename?: 'SettingsQuery', databaseSettings: { __typename?: 'DatabaseSettingsDto', databaseDirectory: string } } };


export const GetDatabaseSettingsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetDatabaseSettings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"databaseSettings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"databaseDirectory"}}]}}]}}]}}]} as unknown as DocumentNode<GetDatabaseSettingsQuery, GetDatabaseSettingsQueryVariables>;