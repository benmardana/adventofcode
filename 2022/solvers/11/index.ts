import { parseMonkey, debug, calculateMonkeyBusiness } from "./lib";

export const partOne = (input: string) => {
  const applyRelief = (item: number) => Math.floor(item / 3);
  const monkeys = input.split("\n\n").map(parseMonkey);

  const rounds = 20;
  let round = 1;

  while (round <= rounds) {
    monkeys.forEach(({ items, inspect, test }, index) => {
      debug(`Monkey ${index}`);
      while (items[0]) {
        const item = items.shift()!;
        debug(`  Monkey inspects an item with a worry level of ${item}`);
        const handledItem = inspect(item);
        debug(`    Worry level is multiplied by X to ${handledItem}.`);
        const worryLevel = applyRelief(handledItem);
        debug(
          `    Monkey gets bored with item. Worry level is divided by X to ${worryLevel}.`
        );
        const target = test(worryLevel);
        debug(
          `    Item with worry level ${worryLevel} is thrown to monkey ${target}.`
        );
        monkeys[target].items.push(worryLevel);
      }
    });

    round++;
  }
  debug(monkeys);
  return calculateMonkeyBusiness(monkeys);
};
export const partTwo = (input: string) => {
  const monkeys = input.split("\n\n").map(parseMonkey);
  const mod = monkeys.reduce((acc, b) => acc * b.divisor, 1);
  const applyRelief = (item: number) => item % mod;

  const rounds = 10000;
  let round = 1;

  while (round <= rounds) {
    monkeys.forEach(({ items, inspect, test }, index) => {
      debug(`Monkey ${index}`);
      while (items[0]) {
        const item = items.shift()!;
        debug(`  Monkey inspects an item with a worry level of ${item}`);
        const handledItem = inspect(item);
        debug(`    Worry level is multiplied by X to ${handledItem}.`);
        const worryLevel = applyRelief(handledItem);
        debug(
          `    Monkey gets bored with item. Worry level is divided by X to ${worryLevel}.`
        );
        const target = test(worryLevel);
        debug(
          `    Item with worry level ${worryLevel} is thrown to monkey ${target}.`
        );
        monkeys[target].items.push(worryLevel);
      }
    });

    round++;
  }
  debug(monkeys);
  return calculateMonkeyBusiness(monkeys);
};
