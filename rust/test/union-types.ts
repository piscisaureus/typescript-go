// This file tests union types functionality

// Basic union type declaration
let id: string | number;

// Assigning different values to a union type
id = "abc123"; // Valid - string
id = 42; // Valid - number

// Function that accepts a union type
function printId(id: string | number) {
  console.log(id);
}

// Valid function calls
printId("abc123");
printId(42);

// Invalid function call - boolean is not part of the union
printId(true);

// Union with more than two types
let status: string | number | boolean;
status = "active";
status = 1;
status = true;
status = {}; // Invalid - object is not part of the union

// Array of union types
let codes: (string | number)[];
codes = ["A101", 202, "B303", 404];
codes.push(true); // Invalid - boolean is not string | number

// Object with union type properties
interface Response {
  data: string | null;
  status: number | string;
}

// Union with interface types
interface Success {
  success: true;
  data: string;
}

interface Error {
  success: false;
  error: string;
}

let result: Success | Error;

// Valid assignment - matches Success interface
result = {
  success: true,
  data: "Operation completed",
};

// Valid assignment - matches Error interface
result = {
  success: false,
  error: "Operation failed",
};

// Invalid assignment - doesn't match either interface
result = {
  status: "ok",
};
