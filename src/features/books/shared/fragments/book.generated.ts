// @ts-nocheck
import type * as Types from '../../../../generated/graphql';

import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type BookCardFragment = { __typename?: 'BookDto', id: number, title: string, author?: string | null };

export type BookDetailFragment = { __typename?: 'BookDto', id: number, title: string, author?: string | null, description?: string | null, publishedYear?: number | null };

export const BookCardFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"BookCard"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"BookDto"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}}]}}]} as unknown as DocumentNode<BookCardFragment, unknown>;
export const BookDetailFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"BookDetail"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"BookDto"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"publishedYear"}}]}}]} as unknown as DocumentNode<BookDetailFragment, unknown>;