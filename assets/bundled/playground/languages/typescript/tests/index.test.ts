function add(a: number, b: number) {
  return a + b;
}

describe("Sample Test", () => {
  it("should add provided numbers", () => {
    expect(add(2, 5)).toBe(7);
  });
});
