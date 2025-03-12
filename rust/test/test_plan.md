# TypeScript Test Development Plan

Based on our test harness, we can see we need to improve our Rust implementation
to match Deno's behavior.

## Current Issues

### Current test files

1. `pass.ts` - Current Result: FAILING
   - Our compiler reports: "Unexpected token: PlusToken"
   - Deno reports: No errors

2. `fail.ts` - Current Result: PASSING
   - Both compilers report errors as expected
   - However, our compiler only reports "Unexpected token: PlusToken" while Deno
     gives more specific type checking errors:
     - TS2554: Expected 4 arguments, but got 1
     - TS2554: Expected 4 arguments, but got 5
     - TS2345: Argument of type '{}' is not assignable to parameter of type
       'any[]'
     - TS2345: Argument of type 'number' is not assignable to parameter of type
       'string'

## Improvement Plan

### 1. Fix Parser & Scanner

- The "Unexpected token: PlusToken" error suggests our parser is having trouble
  with the plus operator in the return statement:
  `return a + b + String(c) + d.join("");`
- Need to improve the parser to properly handle expressions with multiple
  operations
- Need to ensure the scanner correctly handles binary operations

### 2. Improve Type Checking

- Need to implement more detailed type checking to match Deno's error messages
- Implement proper validation for:
  - Incorrect number of arguments
  - Type compatibility checks for arguments
  - Array type checking

### 3. Enhance Error Messages

- Need to improve our error message format to include:
  - Proper TypeScript error codes (TS2554, TS2345, etc.)
  - Specific error messages that explain the problem
  - Line and column information
  - Code snippets showing the error location

## New Test Cases to Add

Once we fix the current issues, we should add more comprehensive test cases to
validate:

1. **Type Annotations**
   - Type annotations for variables
   - Optional parameters in functions
   - Default parameter values
   - Rest parameters

2. **Object Types**
   - Interface declarations
   - Object literal types
   - Property access type checking

3. **Control Flow**
   - Type narrowing in if/else statements
   - Loop constructs

4. **Functions**
   - Arrow functions
   - Function overloads
   - Generic functions

## Roadmap

1. Fix the binary expression (plus operator) issue
2. Improve error reporting to show the correct line/column
3. Update type checking to use proper error codes
4. Create more sophisticated error messages
5. Add more test cases covering additional TypeScript features
