// @ts-nocheck
import type * as Types from '../../../../generated/graphql';

import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type GetAppearanceSettingsQueryVariables = Types.Exact<{ [key: string]: never; }>;


export type GetAppearanceSettingsQuery = { __typename?: 'QueryRoot', settings: { __typename?: 'SettingsQuery', appearanceSettings: { __typename?: 'AppearanceSettingsDto', theme: string } } };


export const GetAppearanceSettingsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetAppearanceSettings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"appearanceSettings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"theme"}}]}}]}}]}}]} as unknown as DocumentNode<GetAppearanceSettingsQuery, GetAppearanceSettingsQueryVariables>;