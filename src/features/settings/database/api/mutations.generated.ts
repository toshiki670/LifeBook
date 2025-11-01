// @ts-nocheck
import type * as Types from '../../../../generated/graphql';

import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type UpdateDatabaseSettingsMutationVariables = Types.Exact<{
  databaseDirectory?: Types.InputMaybe<Types.Scalars['String']['input']>;
}>;


export type UpdateDatabaseSettingsMutation = { __typename?: 'MutationRoot', settings: { __typename?: 'SettingsMutation', updateDatabaseSettings: { __typename?: 'DatabaseSettingsDto', databaseDirectory: string } } };


export const UpdateDatabaseSettingsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateDatabaseSettings"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"databaseDirectory"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"settings"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateDatabaseSettings"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"databaseDirectory"},"value":{"kind":"Variable","name":{"kind":"Name","value":"databaseDirectory"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"databaseDirectory"}}]}}]}}]}}]} as unknown as DocumentNode<UpdateDatabaseSettingsMutation, UpdateDatabaseSettingsMutationVariables>;