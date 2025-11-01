// @ts-nocheck
import type * as Types from '../../../../generated/graphql';

import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type GetGeneralSettingsQueryVariables = Types.Exact<{ [key: string]: never; }>;


export type GetGeneralSettingsQuery = { __typename?: 'QueryRoot', settings: { __typename?: 'SettingsQuery', generalSettings: { __typename?: 'GeneralSettingsDto', language: string } } };


export const GetGeneralSettingsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetGeneralSettings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"generalSettings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"language"}}]}}]}}]}}]} as unknown as DocumentNode<GetGeneralSettingsQuery, GetGeneralSettingsQueryVariables>;