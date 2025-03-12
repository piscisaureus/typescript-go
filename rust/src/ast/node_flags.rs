// Corresponds to internal/ast/nodeflags.go in the Go implementation

use std::ops::{BitAnd, BitOr};

/// NodeFlags is a set of flags used on AST nodes.
/// In Go, this is defined as:
/// ```go
/// type NodeFlags uint32
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeFlags {
    None = 0,
    Let = 1 << 0,
    Const = 1 << 1,
    NestedNamespace = 1 << 2,
    Synthesized = 1 << 3,
    Namespace = 1 << 4,
    OptionalChain = 1 << 5,
    ExportContext = 1 << 6,
    ContainsThis = 1 << 7,
    HasImplicitReturn = 1 << 8,
    HasExplicitReturn = 1 << 9,
    GlobalAugmentation = 1 << 10,
    HasAsyncFunctions = 1 << 11,
    DisallowInContext = 1 << 12,
    YieldContext = 1 << 13,
    DecoratorContext = 1 << 14,
    AwaitContext = 1 << 15,
    ThisNodeHasError = 1 << 16,
    JavaScriptFile = 1 << 17,
    ThisNodeOrAnySubNodesHasError = 1 << 18,
    HasAggregatedChildData = 1 << 19,
    JSDoc = 1 << 20,
    // Some flags are used to track if a subtree contains a specific kind of node.
    // This is primarily used by transformations to determine if a subtree needs to be visited.
    PossiblyContainsDynamicImport = 1 << 21,
    PossiblyContainsImportMeta = 1 << 22,
    // TODO: Add more flags as needed
}

impl BitOr for NodeFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        // Safe to transmute as we're treating these as bit flags
        let result = (self as u32) | (rhs as u32);
        unsafe { std::mem::transmute(result) }
    }
}

impl BitAnd for NodeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let result = (self as u32) & (rhs as u32);
        unsafe { std::mem::transmute(result) }
    }
}
