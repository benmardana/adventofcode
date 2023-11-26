const DEBUG = false;

export const debug = (log: any) => DEBUG && console.log(log);

interface Monkey {
  items: number[];
  inspect: (item: number) => number;
  test: (item: number) => number;
  inspections: {
    value: number;
  };
  divisor: number;
}

const Monkey = (
  items: number[],
  operation: (item: number) => number,
  test: (item: number) => number,
  divisor: number
) => {
  let inspections = { value: 0 };
  const inspect = (item: number) => {
    debug(inspections);
    inspections.value = inspections.value + 1;
    return operation(item);
  };
  return {
    items,
    inspect,
    test,
    divisor,
    inspections,
  } as Monkey;
};

const parseOperation = (operator: string, operand?: number) => {
  switch (operator) {
    case "*":
      return (item: number) => (operand ? item * operand : item * item);
    case "+":
      return (item: number) => (operand ? item + operand : item + item);
    case "-":
      return (item: number) => (operand ? item - operand : item - item);
    case "/":
      return (item: number) => (operand ? item / operand : item / item);
    default:
      return (item: number) => item;
  }
};

export const parseMonkey = (monkeyString: string) => {
  const [, itemLine, operationLine, testLine, trueLine, falseLine] =
    monkeyString.split("\n");

  const items = itemLine.slice(18).split(", ").map(Number);
  const [operator, subject] = operationLine.slice(23).split(" ");
  const divisor = Number(testLine.slice(21));
  const truthy = Number(trueLine.at(-1));
  const falsy = Number(falseLine.at(-1));

  const operation = parseOperation(
    operator,
    subject === "old" ? undefined : Number(subject)
  );
  const test = (item: number) => {
    const result = item % divisor === 0;

    if (result) {
      debug(`    Current worry level is divisible by ${divisor}.`);
      return truthy;
    }
    debug(`    Current worry level is not divisible by ${divisor}.`);
    return falsy;
  };

  return Monkey(items, operation, test, divisor);
};

export const calculateMonkeyBusiness = (monkeys: Monkey[]) => {
  const [monkeyA, monkeyB] = monkeys
    .sort((a, b) => b.inspections.value - a.inspections.value)
    .slice(0, 2)
    .map(({ inspections: { value } }) => value);

  return monkeyA * monkeyB;
};
