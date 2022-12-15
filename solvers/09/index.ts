import { adjacent, Coordinate } from "./lib";

const example = `R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2`;

interface Rope {
  head: Coordinate[];
  tail: Coordinate;
}
const move = (
  [x, y]: Coordinate,
  direction: string,
  magnitude: number
): Coordinate => {
  switch (direction) {
    case "U":
      return [x, y + magnitude];
    case "D":
      return [x, y - magnitude];
    case "L":
      return [x - magnitude, y];
    case "R":
      return [x + magnitude, y];
    default:
      return [x, y];
  }
};

export const partOne = (input: string) => {
  const visitedTailPositions: Record<string, number> = {};

  const rope: Rope = {
    head: [[0, 0]],
    tail: [0, 0],
  };

  const visitPosition = (coordinate: Coordinate) => {
    const visited = visitedTailPositions[`${coordinate[0]}-${coordinate[1]}`];
    visitedTailPositions[`${coordinate[0]}-${coordinate[1]}`] = visited
      ? visited + 1
      : 1;
  };

  input.split("\n").forEach((line) => {
    const [direction, magnitude] = line.split(" ");
    for (let index = 0; index < Number(magnitude); index++) {
      visitPosition(rope.tail);
      rope.head.unshift(move(rope.head[0], direction, 1));
      if (!adjacent(rope.head[0], rope.tail)) {
        rope.tail = rope.head[1];
      }
    }
  });
  return Object.keys(visitedTailPositions).length;
};

const example2 = `R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20`;
export const partTwo = (input: string) => {
  const visitedTailPositions: Record<string, number> = {};

  const rope: Rope = {
    head: [[0, 0]],
    tail: [0, 0],
  };

  const visitPosition = (coordinate: Coordinate) => {
    const visited = visitedTailPositions[`${coordinate[0]}-${coordinate[1]}`];
    visitedTailPositions[`${coordinate[0]}-${coordinate[1]}`] = visited
      ? visited + 1
      : 1;
  };

  example2.split("\n").forEach((line) => {
    const [direction, magnitude] = line.split(" ");
    for (let index = 0; index < Number(magnitude); index++) {
      visitPosition(rope.tail);
      rope.head.unshift(move(rope.head[0], direction, 1));
      console.log(rope.head);
      if (rope.head[8] && !adjacent(rope.head[8], rope.tail)) {
        rope.tail = rope.head[9];
      }
    }
  });
  return Object.keys(visitedTailPositions).length;
};
