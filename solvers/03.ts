const intersection = (a: string, b: string, c?: string) => {
  const checkMapB = Object.fromEntries(b.split("").map((el) => [el, el]));
  const checkMapC = c
    ? Object.fromEntries(c.split("").map((el) => [el, el]))
    : undefined;
  for (const element of a.split("")) {
    if (
      checkMapC ? checkMapB[element] && checkMapC[element] : checkMapB[element]
    ) {
      return element;
    }
  }
  return "undefined";
};

const chunk = (arr: string[], len: number) => {
  const chunks = [];
  let i = 0;

  while (i < arr.length) {
    chunks.push(arr.slice(i, (i += len)));
  }

  return chunks;
};

const charMap: Record<string, number> = {
  a: 1,
  b: 2,
  c: 3,
  d: 4,
  e: 5,
  f: 6,
  g: 7,
  h: 8,
  i: 9,
  j: 10,
  k: 11,
  l: 12,
  m: 13,
  n: 14,
  o: 15,
  p: 16,
  q: 17,
  r: 18,
  s: 19,
  t: 20,
  u: 21,
  v: 22,
  w: 23,
  x: 24,
  y: 25,
  z: 26,
  A: 27,
  B: 28,
  C: 29,
  D: 30,
  E: 31,
  F: 32,
  G: 33,
  H: 34,
  I: 35,
  J: 36,
  K: 37,
  L: 38,
  M: 39,
  N: 40,
  O: 41,
  P: 42,
  Q: 43,
  R: 44,
  S: 45,
  T: 46,
  U: 47,
  V: 48,
  W: 49,
  X: 50,
  Y: 51,
  Z: 52,
};

export const partOne = (input: string) =>
  input
    .split("\n")
    .map(
      (line) =>
        charMap[
          intersection(line.slice(0, line.length), line.slice(line.length / 2))
        ]
    )
    .reduce((acc, curr) => acc + curr, 0);

export const partTwo = (input: string) =>
  chunk(input.split("\n"), 3)
    .map(([a, b, c]) => charMap[intersection(a, b, c)])
    .reduce((acc, curr) => acc + curr, 0);
