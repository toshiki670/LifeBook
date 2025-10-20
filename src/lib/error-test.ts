/**
 * This file contains intentional errors for CI failure testing
 * DO NOT MERGE
 */

// TypeScript error: using unknown variable
export function brokenFunction() {
	return unknownVariable + 123;
}

// Biome format error: inconsistent formatting
export const badFormat={value:1,name:"test",data:[1,2,3]};

// Unused variable (lint error)
const unusedVariable = "This will trigger lint error";

