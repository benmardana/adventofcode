const CPU = () => {
  const register = {
    X: 1,
  };

  const instructions = {
    addX: (num: number) => () => {
      register.X = register.X + num;
    },
    noop: () => {},
  };

  return { register, ...instructions };
};

const signalStrength = (cycle: number, registerX: number) => cycle * registerX;

const isInterestingCycle = (cycle: number) => {
  const remainder = (cycle - 20) % 40;
  return Number.isNaN(remainder) || remainder === 0;
};

const chunk = (arr: string[], len: number) => {
  const chunks = [];
  let i = 0;

  while (i < arr.length) {
    chunks.push(arr.slice(i, (i += len)));
  }

  return chunks;
};

export const partOne = (input: string) => {
  const cpu = CPU();
  const executionStack: (() => void)[] = [];

  const lines = input.split("\n");

  let lineIndex = 0;
  let cycle = 1;

  const interestingValues: number[] = [];

  do {
    // console.log(`beginning cycle ${cycle}`);
    if (!executionStack.length) {
      const [instruction, value] = lines[lineIndex++].split(" ");
      if (instruction === "noop") {
        executionStack.push(cpu.noop);
      } else {
        executionStack.push(() => executionStack.push(cpu.addX(Number(value))));
      }
    }

    if (isInterestingCycle(cycle)) {
      interestingValues.push(signalStrength(cycle, cpu.register.X));
    }
    executionStack.pop()?.();

    // console.log(`ending cycle ${cycle}`);
    cycle++;
  } while (executionStack.length > 0 || lines[lineIndex]);

  return interestingValues.reduce((acc, v) => acc + v, 0);
};

const pixel = (cycle: number, registerX: number) => {
  const visible =
    registerX - 1 === cycle - 1 ||
    registerX === cycle - 1 ||
    registerX + 1 === cycle - 1;

  return visible ? "#" : ".";
};

const example = `addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop`;

export const partTwo = (input: string) => {
  const cpu = CPU();
  const executionStack: (() => void)[] = [];

  const lines = input.split("\n");

  let lineIndex = 0;
  let cycle = 1;

  const CRT: string[] = [];

  do {
    console.log(`beginning cycle ${cycle} .. sprite at ${cpu.register.X}`);
    if (!executionStack.length) {
      const [instruction, value] = lines[lineIndex++].split(" ");
      if (instruction === "noop") {
        executionStack.push(cpu.noop);
      } else {
        executionStack.push(() => executionStack.push(cpu.addX(Number(value))));
      }
    }

    CRT.push(pixel(cycle % 40, cpu.register.X));
    executionStack.pop()?.();

    console.log(`ending cycle ${cycle} .. sprite at ${cpu.register.X}`);
    cycle++;
  } while (executionStack.length > 0 || lines[lineIndex]);

  return (
    "\n" +
    chunk(CRT, 40)
      .map((line) => line.join(""))
      .join("\n")
  );
};
