// This file tests object literal expressions

// Define a simpler test that our parser can handle better
function simpleObjectTest() {
  // Simple object creation
  const user = {
    name: "John",
    age: 30,
    isActive: true,
  };

  // Nested object
  const nestedObj = {
    id: 1,
    data: {
      value: 42,
      text: "hello",
    },
  };

  // Empty object
  const empty = {};

  // Object with method-like property (function)
  const objWithFunction = {
    id: 123,
    process: function (input: number) {
      return input + 1;
    },
  };

  return {
    user: user,
    nested: nestedObj,
    empty: empty,
    func: objWithFunction,
  };
}

// Create an object and access properties
const testObj = simpleObjectTest();
console.log(testObj.user.name);
console.log(testObj.nested.data.value);

// Merge objects
const merged = {
  ...testObj.user,
  location: "New York",
};

// Function that takes an object parameter
function processUser(user: { name: string; age: number }): string {
  return user.name + " is " + user.age + " years old";
}

// Call the function with valid object
processUser({ name: "Jane", age: 25 });
