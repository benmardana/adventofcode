export type Coordinate = [number, number];

export const adjacent = ([ax, ay]: Coordinate, [bx, by]: Coordinate) => {
  return Math.abs(ax - bx) <= 1 && Math.abs(ay - by) <= 1;
};
