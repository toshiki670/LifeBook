// @ts-nocheck
import type * as Types from '../../../../generated/graphql';

import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type UpdateAppearanceSettingsMutationVariables = Types.Exact<{
  theme?: Types.InputMaybe<Types.Scalars['String']['input']>;
}>;


export type UpdateAppearanceSettingsMutation = { __typename?: 'MutationRoot', settings: { __typename?: 'SettingsMutation', updateAppearanceSettings: { __typename?: 'AppearanceSettingsDto', theme: string } } };


export const UpdateAppearanceSettingsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateAppearanceSettings"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"theme"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateAppearanceSettings"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"theme"},"value":{"kind":"Variable","name":{"kind":"Name","value":"theme"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"theme"}}]}}]}}]}}]} as unknown as DocumentNode<UpdateAppearanceSettingsMutation, UpdateAppearanceSettingsMutationVariables>;