// @ts-nocheck
import type * as Types from '../../../../generated/graphql';

import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type CardFragment = { __typename?: 'BookDto', id: number, title: string, author?: string | null };

export type DetailFragment = { __typename?: 'BookDto', id: number, title: string, author?: string | null, description?: string | null, publishedYear?: number | null };

export const CardFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"Card"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"BookDto"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}}]}}]} as unknown as DocumentNode<CardFragment, unknown>;
export const DetailFragmentDoc = {"kind":"Document","definitions":[{"kind":"FragmentDefinition","name":{"kind":"Name","value":"Detail"},"typeCondition":{"kind":"NamedType","name":{"kind":"Name","value":"BookDto"}},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"author"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"publishedYear"}}]}}]} as unknown as DocumentNode<DetailFragment, unknown>;