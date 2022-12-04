export const partOne = (input: string) =>
  input
    ?.split("\n\n")
    .map((group) =>
      group.split("\n").reduce((acc, curr) => acc + Number(curr), 0)
    )
    .sort((a, b) => b - a)[0];

export const partTwo = (input: string) => {
  const parsedInput = input
    ?.split("\n\n")
    .map((group) =>
      group.split("\n").reduce((acc, curr) => acc + Number(curr), 0)
    )
    .sort((a, b) => b - a);

  return [parsedInput[0], parsedInput[1], parsedInput[2]].reduce(
    (acc, curr) => acc + curr,
    0
  );
};
