import type { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core"
import type * as Types from "../../generated/graphql"
export type SettingsUpdateGeneralMutationVariables = Types.Exact<{
  language: Types.Scalars["String"]["input"]
}>

export type SettingsUpdateGeneralMutation = {
  __typename?: "MutationRoot"
  settings: {
    __typename?: "SettingsMutation"
    updateGeneralSettings: { __typename?: "GeneralSettingsDto"; language: string }
  }
}

export type SettingsUpdateAppearanceMutationVariables = Types.Exact<{
  theme: Types.Scalars["String"]["input"]
}>

export type SettingsUpdateAppearanceMutation = {
  __typename?: "MutationRoot"
  settings: {
    __typename?: "SettingsMutation"
    updateAppearanceSettings: { __typename?: "AppearanceSettingsDto"; theme: string }
  }
}

export type SettingsUpdateDatabaseMutationVariables = Types.Exact<{
  databaseDirectory: Types.Scalars["String"]["input"]
}>

export type SettingsUpdateDatabaseMutation = {
  __typename?: "MutationRoot"
  settings: {
    __typename?: "SettingsMutation"
    updateDatabaseSettings: { __typename?: "DatabaseSettingsDto"; databaseDirectory: string }
  }
}

export const SettingsUpdateGeneralDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "SettingsUpdateGeneral" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "language" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "settings" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "updateGeneralSettings" },
                  arguments: [
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "language" },
                      value: { kind: "Variable", name: { kind: "Name", value: "language" } },
                    },
                  ],
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [{ kind: "Field", name: { kind: "Name", value: "language" } }],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<SettingsUpdateGeneralMutation, SettingsUpdateGeneralMutationVariables>
export const SettingsUpdateAppearanceDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "SettingsUpdateAppearance" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "theme" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "settings" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "updateAppearanceSettings" },
                  arguments: [
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "theme" },
                      value: { kind: "Variable", name: { kind: "Name", value: "theme" } },
                    },
                  ],
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [{ kind: "Field", name: { kind: "Name", value: "theme" } }],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<
  SettingsUpdateAppearanceMutation,
  SettingsUpdateAppearanceMutationVariables
>
export const SettingsUpdateDatabaseDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "SettingsUpdateDatabase" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "databaseDirectory" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "settings" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "updateDatabaseSettings" },
                  arguments: [
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "databaseDirectory" },
                      value: {
                        kind: "Variable",
                        name: { kind: "Name", value: "databaseDirectory" },
                      },
                    },
                  ],
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "databaseDirectory" } },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<
  SettingsUpdateDatabaseMutation,
  SettingsUpdateDatabaseMutationVariables
>
