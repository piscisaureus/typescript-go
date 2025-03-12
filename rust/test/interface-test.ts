// Interface test
interface Point {
  x: number;
  y: number;
}

interface LabeledPoint {
  x: number;
  y: number;
  label: string;
}

// Valid assignments
const p1: Point = { x: 10, y: 20 }; // OK

// Invalid assignment - missing property
const p3: Point = { x: 10 }; // Should fail, missing y

// Valid assignment to labeled point
const lp1: LabeledPoint = { x: 10, y: 20, label: "test" }; // OK

// Invalid assignment to labeled point - missing property
const lp2: LabeledPoint = { x: 10, y: 20 }; // Should fail, missing label

// Invalid assignment - wrong type
const p5: Point = { x: "10", y: 20 }; // Should fail, x is not a number

// Valid object-to-object assignment
const p4: Point = p1; // OK
