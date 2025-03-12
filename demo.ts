function demo(a: string, b: number, c: boolean, d: any[]): string {
  return a + b + String(c) + d.join("");
}

// This should work correctly
demo("hello", 42, true, ["world"]);

// This should fail
// demo("hello");
// demo("hello", 0, true, ["world"], "extra");
// demo("hello", 0, true, {});
// demo(0, 0, true, []);
