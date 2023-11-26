import { parseInputToSquares, Square, north, east, south, west } from "./lib";

const pathLength = (cameFrom: Record<string, string>, current: string) => {
  let pathLength = 1;
  while (cameFrom[current]) {
    console.log(current);
    current = cameFrom[current];
    pathLength = pathLength + 1;
  }
  return pathLength;
};

export const partOne = (input: string) => {
  const { squares, start, end } = parseInputToSquares(input);

  const cameFrom: Record<string, string> = {};
  const open: Record<string, Square> = {
    [start.index]: start,
  };
  const gScore: Record<string, number> = {};
  const fScore: Record<string, number> = {};

  squares.forEach((row) => {
    row.forEach((square) => {
      gScore[square.index] = Infinity;
      fScore[square.index] = Infinity;
    });
  });

  gScore[start.index] = 0;
  fScore[start.index] = 0;

  while (Object.keys(open).length) {
    const current = Object.values(open)
      .sort((a, b) => fScore[a.index] - fScore[b.index])
      .shift()!;

    delete open[current.index];
    if (current.isEnd) {
      console.log(Object.keys(cameFrom).length);
      return pathLength(cameFrom, current.index) - 1;
    }

    const neighbours = [
      north(squares, current),
      east(squares, current),
      south(squares, current),
      west(squares, current),
    ];

    neighbours.forEach((neighbour) => {
      if (!neighbour) return undefined;

      const d = neighbour.height - current.height;
      const tentativeGScore = gScore[current.index] + d;

      if (tentativeGScore < gScore[neighbour.index]) {
        const h =
          Math.abs(neighbour.coordinate[0] - end.coordinate[0]) +
          Math.abs(neighbour.coordinate[1] - end.coordinate[1]);

        cameFrom[neighbour.index] = current.index;
        gScore[neighbour.index] = tentativeGScore;
        fScore[neighbour.index] = tentativeGScore + h;

        if (!open[neighbour.index]) {
          open[neighbour.index] = neighbour;
        }
      }
    });
  }
};

export const partTwo = (input: string) => {
  // return input;
};
