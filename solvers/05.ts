const chunk = (arr: string[], len: number) => {
  const chunks = [];
  let i = 0;

  while (i < arr.length) {
    chunks.push(arr.slice(i, (i += len)));
  }

  return chunks;
};

const getCrates = (input: string) => {
  const crateLines = input
    .split("\n")
    .slice(0, -1)
    .map((line) => chunk(line.split(""), 4).map((crate) => crate[1]));

  return crateLines[0].map((_, colIndex) =>
    crateLines
      .map((row) => row[colIndex])
      .filter((el) => el !== " ")
      .reverse()
  );
};

const moveCrates = (move: number, from: string[], to: string[]) => {
  for (let curr = 0; curr < move; curr++) {
    const crate = from.pop();
    if (crate) {
      to.push(crate);
    }
  }
};

const moveCratesTogether = (move: number, from: string[], to: string[]) => {
  const crates: string[] = [];
  for (let curr = 0; curr < move; curr++) {
    const crate = from.pop();
    if (crate) {
      crates.unshift(crate);
    }
  }
  to.push(...crates);
};

export const partOne = (input: string) => {
  const [cratesInput, procedureInput] = input.split("\n\n");

  const crates = getCrates(cratesInput);
  for (const procedure of procedureInput.split("\n")) {
    const [, move, , from, , to] = procedure.split(" ");
    moveCrates(Number(move), crates[Number(from) - 1], crates[Number(to) - 1]);
  }
  return crates.map((stack) => stack.at(-1)).join("");
};

export const partTwo = (input: string) => {
  const [cratesInput, procedureInput] = input.split("\n\n");

  const crates = getCrates(cratesInput);
  for (const procedure of procedureInput.split("\n")) {
    const [, move, , from, , to] = procedure.split(" ");
    moveCratesTogether(
      Number(move),
      crates[Number(from) - 1],
      crates[Number(to) - 1]
    );
  }
  return crates.map((stack) => stack.at(-1)).join("");
};
