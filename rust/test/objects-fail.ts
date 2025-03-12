// This file tests object literal type errors

// Define a simple interface for typing
interface User {
  name: string;
  age: number;
  isActive: boolean;
  email: string; // Required field
}

// Missing required property 'email'
const missingProperty: User = {
  name: "John",
  age: 30,
  isActive: true,
};

// Type mismatch on property
const wrongPropertyType: User = {
  name: "Alice",
  age: "twenty-nine", // String instead of number
  isActive: true,
  email: "alice@example.com",
};

// Extra property not in interface
const extraProperty: User = {
  name: "Bob",
  age: 25,
  isActive: true,
  email: "bob@example.com",
  location: "New York", // Extra property
};

// Function expecting an object with specific shape
function processUser(user: { name: string; age: number }): string {
  return user.name + " is " + user.age + " years old";
}

// Function with wrong argument types
processUser("not an object");

// Function with missing property
processUser({ name: "Jane" });

// Object destructuring with type errors
const { name, age, unknownProp } = { name: "Charlie", age: 40 };

// Accessing non-existent property
const obj = { x: 10, y: 20 };
const value = obj.z; // Non-existent property

// Nested object with type errors
const nestedObj = {
  id: 1,
  data: {
    value: 42,
    text: 123, // Should be string
  },
};

// Attempt to use string methods on a number
const invalidMethodCall = nestedObj.data.value.toLowerCase();

// Object assigned to wrong type
const numbers: number[] = {
  one: 1,
  two: 2,
};
