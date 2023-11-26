const contains = (a: number[], b: number[]) => {
  const [aLower, aUpper] = a;
  const [bLower, bUpper] = b;

  if (aLower <= bLower && aUpper >= bUpper) {
    return true;
  }
  if (bLower <= aLower && bUpper >= aUpper) {
    return true;
  }
  return false;
};

const overlap = (a: number[], b: number[]) => {
  const [aLower, aUpper] = a;
  const [bLower, bUpper] = b;

  return aLower <= bUpper && bLower <= aUpper;
};

export const partOne = (input: string) =>
  input.split("\n").filter((line) => {
    const [a, b] = line.split(",").map((range) => range.split("-").map(Number));

    return contains(a, b) || contains(b, a);
  }).length;

export const partTwo = (input: string) =>
  input.split("\n").filter((line) => {
    const [a, b] = line.split(",").map((range) => range.split("-").map(Number));

    return overlap(a, b);
  }).length;
