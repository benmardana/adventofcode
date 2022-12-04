import assert from "assert";
import { getInput } from "./input/getInput";

const day = process.argv[2] ? Number(process.argv[2]) : null;
assert(day, "provide a day to run (1, 2, 3, ...)");

const input = await getInput(day);

try {
  const solver = await import(
    `${import.meta.dir}/solvers/${day.toString().padStart(2, "0")}.ts`
  );
  solver.default(input);
} catch {
  console.log("not implemented");
}
