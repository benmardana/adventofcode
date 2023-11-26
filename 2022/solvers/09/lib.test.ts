import { expect, it, describe } from "vitest";
import { adjacent } from "./lib";

describe("adjacent", () => {
  it("should calculate adjacency", () => {
    expect(adjacent([0, 0], [0, 0])).toEqual(true);
    expect(adjacent([0, 0], [-1, 1])).toEqual(true);
    expect(adjacent([0, 0], [0, 1])).toEqual(true);
    expect(adjacent([0, 0], [1, 1])).toEqual(true);
    expect(adjacent([0, 0], [-1, 0])).toEqual(true);
    expect(adjacent([0, 0], [1, 0])).toEqual(true);
    expect(adjacent([0, 0], [-1, -1])).toEqual(true);
    expect(adjacent([0, 0], [0, -1])).toEqual(true);
    expect(adjacent([0, 0], [1, -1])).toEqual(true);
  });

  it("should calculate non adjacency", () => {
    expect(adjacent([0, 0], [2, 2])).toEqual(false);
    expect(adjacent([0, 0], [-2, 2])).toEqual(false);
    expect(adjacent([0, 0], [0, 2])).toEqual(false);
    expect(adjacent([0, 0], [2, 2])).toEqual(false);
    expect(adjacent([0, 0], [-2, 0])).toEqual(false);
    expect(adjacent([0, 0], [2, 0])).toEqual(false);
    expect(adjacent([0, 0], [-2, -2])).toEqual(false);
    expect(adjacent([0, 0], [0, -2])).toEqual(false);
    expect(adjacent([0, 0], [2, -2])).toEqual(false);
  });
});
