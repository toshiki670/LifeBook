import type { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core"
import type * as Types from "../../generated/graphql"
export type SettingsGetGeneralQueryVariables = Types.Exact<{ [key: string]: never }>

export type SettingsGetGeneralQuery = {
  __typename?: "QueryRoot"
  settings: {
    __typename?: "SettingsQuery"
    generalSettings: { __typename?: "GeneralSettingsDto"; language: string }
  }
}

export type SettingsGetAppearanceQueryVariables = Types.Exact<{ [key: string]: never }>

export type SettingsGetAppearanceQuery = {
  __typename?: "QueryRoot"
  settings: {
    __typename?: "SettingsQuery"
    appearanceSettings: { __typename?: "AppearanceSettingsDto"; theme: string }
  }
}

export type SettingsGetDatabaseQueryVariables = Types.Exact<{ [key: string]: never }>

export type SettingsGetDatabaseQuery = {
  __typename?: "QueryRoot"
  settings: {
    __typename?: "SettingsQuery"
    databaseSettings: { __typename?: "DatabaseSettingsDto"; databaseDirectory: string }
  }
}

export const SettingsGetGeneralDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "SettingsGetGeneral" },
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
                  name: { kind: "Name", value: "generalSettings" },
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
} as unknown as DocumentNode<SettingsGetGeneralQuery, SettingsGetGeneralQueryVariables>
export const SettingsGetAppearanceDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "SettingsGetAppearance" },
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
                  name: { kind: "Name", value: "appearanceSettings" },
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
} as unknown as DocumentNode<SettingsGetAppearanceQuery, SettingsGetAppearanceQueryVariables>
export const SettingsGetDatabaseDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "SettingsGetDatabase" },
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
                  name: { kind: "Name", value: "databaseSettings" },
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
} as unknown as DocumentNode<SettingsGetDatabaseQuery, SettingsGetDatabaseQueryVariables>
