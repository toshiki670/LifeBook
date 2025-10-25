import type { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core"
import type * as Types from "../../generated/graphql"
export type BooksGetAllQueryVariables = Types.Exact<{ [key: string]: never }>

export type BooksGetAllQuery = {
  __typename?: "QueryRoot"
  library: {
    __typename?: "BookQuery"
    books: Array<{
      __typename?: "BookDto"
      id: number
      title: string
      author?: string | null
      description?: string | null
      publishedYear?: number | null
    }>
  }
}

export type BooksGetOneQueryVariables = Types.Exact<{
  id: Types.Scalars["Int"]["input"]
}>

export type BooksGetOneQuery = {
  __typename?: "QueryRoot"
  library: {
    __typename?: "BookQuery"
    book?: {
      __typename?: "BookDto"
      id: number
      title: string
      author?: string | null
      description?: string | null
      publishedYear?: number | null
    } | null
  }
}

export const BooksGetAllDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "BooksGetAll" },
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
                  name: { kind: "Name", value: "books" },
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
} as unknown as DocumentNode<BooksGetAllQuery, BooksGetAllQueryVariables>
export const BooksGetOneDocument = {
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "BooksGetOne" },
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
                  name: { kind: "Name", value: "book" },
                  arguments: [
                    {
                      kind: "Argument",
                      name: { kind: "Name", value: "id" },
                      value: { kind: "Variable", name: { kind: "Name", value: "id" } },
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
} as unknown as DocumentNode<BooksGetOneQuery, BooksGetOneQueryVariables>
