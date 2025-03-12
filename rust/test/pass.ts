function demo(a: string, b: number, c: boolean, d: any[]): string {
  return a + b + String(c) + d.join("");
}

demo("hello", 42, true, ["world"]);
