import type { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core"
import type * as Types from "../../generated/graphql"
export type BooksCreateMutationVariables = Types.Exact<{
  title: Types.Scalars["String"]["input"]
  author?: Types.InputMaybe<Types.Scalars["String"]["input"]>
  description?: Types.InputMaybe<Types.Scalars["String"]["input"]>
  publishedYear?: Types.InputMaybe<Types.Scalars["Int"]["input"]>
}>

export type BooksCreateMutation = {
  __typename?: "MutationRoot"
  library: {
    __typename?: "BookMutation"
    createBook: {
      __typename?: "BookDto"
      id: number
      title: string
      author?: string | null
      description?: string | null
      publishedYear?: number | null
    }
  }
}

export type BooksUpdateMutationVariables = Types.Exact<{
  id: Types.Scalars["Int"]["input"]
  title?: Types.InputMaybe<Types.Scalars["String"]["input"]>
  author?: Types.InputMaybe<Types.Scalars["String"]["input"]>
  description?: Types.InputMaybe<Types.Scalars["String"]["input"]>
  publishedYear?: Types.InputMaybe<Types.Scalars["Int"]["input"]>
}>

export type BooksUpdateMutation = {
  __typename?: "MutationRoot"
  library: {
    __typename?: "BookMutation"
    updateBook: {
      __typename?: "BookDto"
      id: number
      title: string
      author?: string | null
      description?: string | null
      publishedYear?: number | null
    }
  }
}

export type BooksDeleteMutationVariables = Types.Exact<{
  id: Types.Scalars["Int"]["input"]
}>

export type BooksDeleteMutation = {
  __typename?: "MutationRoot"
  library: { __typename?: "BookMutation"; deleteBook: boolean }
}

export const BooksCreateDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "BooksCreate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "title" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
          },
        },
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "author" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
        },
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "description" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
        },
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "publishedYear" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "Int" } },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "library" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "createBook" },
                  arguments: [
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "title" },
                      value: { kind: "Variable", name: { kind: "Name", value: "title" } },
                    },
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "author" },
                      value: { kind: "Variable", name: { kind: "Name", value: "author" } },
                    },
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "description" },
                      value: { kind: "Variable", name: { kind: "Name", value: "description" } },
                    },
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "publishedYear" },
                      value: { kind: "Variable", name: { kind: "Name", value: "publishedYear" } },
                    },
                  ],
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "title" } },
                      { kind: "Field", name: { kind: "Name", value: "author" } },
                      { kind: "Field", name: { kind: "Name", value: "description" } },
                      { kind: "Field", name: { kind: "Name", value: "publishedYear" } },
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
} as unknown as DocumentNode<BooksCreateMutation, BooksCreateMutationVariables>
export const BooksUpdateDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "BooksUpdate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "Int" } },
          },
        },
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "title" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
        },
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "author" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
        },
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "description" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
        },
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "publishedYear" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "Int" } },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "library" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "updateBook" },
                  arguments: [
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "id" },
                      value: { kind: "Variable", name: { kind: "Name", value: "id" } },
                    },
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "title" },
                      value: { kind: "Variable", name: { kind: "Name", value: "title" } },
                    },
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "author" },
                      value: { kind: "Variable", name: { kind: "Name", value: "author" } },
                    },
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "description" },
                      value: { kind: "Variable", name: { kind: "Name", value: "description" } },
                    },
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "publishedYear" },
                      value: { kind: "Variable", name: { kind: "Name", value: "publishedYear" } },
                    },
                  ],
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "title" } },
                      { kind: "Field", name: { kind: "Name", value: "author" } },
                      { kind: "Field", name: { kind: "Name", value: "description" } },
                      { kind: "Field", name: { kind: "Name", value: "publishedYear" } },
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
} as unknown as DocumentNode<BooksUpdateMutation, BooksUpdateMutationVariables>
export const BooksDeleteDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "BooksDelete" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "Int" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "library" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "deleteBook" },
                  arguments: [
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "id" },
                      value: { kind: "Variable", name: { kind: "Name", value: "id" } },
                    },
                  ],
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown as DocumentNode<BooksDeleteMutation, BooksDeleteMutationVariables>
