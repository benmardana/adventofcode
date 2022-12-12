import assert from "assert";
import { getInput } from "./input/getInput";

const day = process.argv[2] ? Number(process.argv[2]) : null;
assert(day, "provide a day to run (1, 2, 3, ...)");

try {
  const input = await getInput(day);

  const { partOne, partTwo } = await import(
    `${import.meta.dir}/solvers/${day.toString().padStart(2, "0")}`
  );

  console.log(`Part one: ${partOne?.(input)}`);
  console.log(`Part two: ${partTwo?.(input)}`);
} catch (e) {
  console.error((e as any).message);
}
