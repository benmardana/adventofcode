export const getInput = async (day: number) => {
  const filePath = `${import.meta.dir}/cache/${day
    .toString()
    .padStart(2, "0")}.json`;

  try {
    const cacheResult = await import(filePath);
    return cacheResult.default.input as string;
  } catch {}

  try {
    const response = await fetch(
      `https://adventofcode.com/2022/day/${day}/input`,
      {
        headers: {
          cookie:
            "session=53616c7465645f5f45882496767ae8b886bf6f95f090c903935c5f62495ad9da5ce426e5f4b265a0d0601dde50d2ce81e7168d95d4f01b9d7dadcb8bd4403e87",
        },
      }
    );
    const input = (await response.text()).trimEnd();

    await Bun.write(filePath, JSON.stringify({ input }));
    return input;
  } catch (e) {
    return null;
  }
};
