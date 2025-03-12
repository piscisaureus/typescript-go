function demo(a: string, b: number, c: boolean, d: any[]): string {
  return a + b;
}

// This should work correctly
demo("hello", 42, true, ["world"]);

// Let's uncomment this to see if we get a type error
demo("hello", "42", true, ["world"]);
