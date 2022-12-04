const solver = (input: string) => {
  const parsedInput = input
    ?.split("\n\n")
    .map((group) =>
      group.split("\n").reduce((acc, curr) => acc + Number(curr), 0)
    )
    .sort((a, b) => b - a);

  console.log(`Part one: ${parsedInput[0]}`);
  console.log(
    `Part two: ${[parsedInput[0], parsedInput[1], parsedInput[2]].reduce(
      (acc, curr) => acc + curr,
      0
    )}`
  );
};

export default solver;
