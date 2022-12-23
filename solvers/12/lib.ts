export interface Square {
  coordinate: [number, number];
  index: string;
  height: number;
  isStart: boolean;
  isEnd: boolean;
}

export const directions = {
  north: [-1, 0],
  east: [0, +1],
  south: [+1, 0],
  west: [0, -1],
};

export const north = (graph: Square[][], square: Square) => {
  const newCoord = [
    square.coordinate[0] + directions.north[0],
    square.coordinate[1] + directions.north[1],
  ];
  const newSquare = graph[newCoord[0]]?.[newCoord[1]];
  if (!newSquare) return undefined;
  if (newSquare.height - square.height > 1) return undefined;
  return newSquare;
};

export const east = (graph: Square[][], square: Square) => {
  const newCoord = [
    square.coordinate[0] + directions.east[0],
    square.coordinate[1] + directions.east[1],
  ];
  const newSquare = graph[newCoord[0]]?.[newCoord[1]];
  if (!newSquare) return undefined;
  if (newSquare.height - square.height > 1) return undefined;
  return newSquare;
};

export const south = (graph: Square[][], square: Square) => {
  const newCoord = [
    square.coordinate[0] + directions.south[0],
    square.coordinate[1] + directions.south[1],
  ];
  const newSquare = graph[newCoord[0]]?.[newCoord[1]];
  if (!newSquare) return undefined;
  if (newSquare.height - square.height > 1) return undefined;
  return newSquare;
};

export const west = (graph: Square[][], square: Square) => {
  const newCoord = [
    square.coordinate[0] + directions.west[0],
    square.coordinate[1] + directions.west[1],
  ];
  const newSquare = graph[newCoord[0]]?.[newCoord[1]];
  if (!newSquare) return undefined;
  if (newSquare.height - square.height > 1) return undefined;
  return newSquare;
};

export const parseInputToSquares = (input: string) => {
  let start: Square;
  let end: Square;

  const squares = input.split("\n").map((line, lineIndex) =>
    line.split("").map((char, charIndex) => {
      const coordinate = [lineIndex, charIndex] as [number, number];
      const isStart = char === "S";
      const isEnd = char === "E";
      let height = char.charCodeAt(0) - 97;
      if (isEnd) {
        height = 26;
      }

      if (isStart) {
        height = -1;
      }

      const square = {
        coordinate,
        index: `${lineIndex}${charIndex}`,
        height,
        isStart,
        isEnd,
      } as Square;

      if (isStart) start = { ...square };
      if (isEnd) end = { ...square };

      return square;
    })
  );

  return {
    squares,
    // @ts-ignore
    start,
    // @ts-ignore
    end,
  };
};
