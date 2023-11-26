const getMarker = (input: string, length: number) => {
  for (let index = length; index < input.length; index++) {
    const window = input.slice(index - length, index).split("");
    const allUnique = !window.some((v, i) => window.indexOf(v) < i);
    if (allUnique) {
      return index;
    }
  }
};

export const partOne = (input: string) => getMarker(input, 4);
export const partTwo = (input: string) => getMarker(input, 14);
