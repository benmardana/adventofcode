const USE_EXAMPLE = false;
const PARENT = "..";
const ROOT = "/";
const MAX_SIZE = 100000;
const example = `$ cd /
$ ls
dir a
$ cd a
$ ls
dir a
2 a.txt
$ cd a
$ ls
99999 a.txt`;

const getFileSizeMap = (input: string) => {
  const fileDB: Record<string, number> = {};
  const linesIterator = input.split("\n").entries();
  linesIterator.next();
  let path = ROOT;

  for (const [, line] of linesIterator) {
    const [first, second, third] = line.split(" ");

    if (first === "$") {
      if (second === "ls") {
        continue;
      }
      if (second === "cd") {
        if (third === PARENT) {
          path = path
            .split("/")
            .slice(0, path.length - 3)
            .join("/");
          if (path === "") {
            path = ROOT;
          }
          continue;
        }
        if (third === ROOT) {
          path = third;
          continue;
        }
        path = path + `${third}/`;
        continue;
      }
    }
    if (first === "dir") {
      continue;
    }

    const [size, filename] = line.split(" ");
    fileDB[`${path}${filename}`] = Number(size);
  }
  return fileDB;
};

const getDirectorySizeMap = (input: Record<string, number>) => {
  let directoryDB: Record<string, number> = {};

  Object.entries(input).forEach(([path, size]) => {
    const directories = path.split("/");
    directories.splice(-1);
    directories[0] = "/";
    directories.forEach((directory, index) => {
      const path = directories.slice(0, index);
      let uniquePath = [...path, directory].join("/");
      if (uniquePath.slice(0, 2) === "//") {
        uniquePath = uniquePath.slice(1);
      }
      directoryDB[uniquePath] = directoryDB[uniquePath]
        ? directoryDB[uniquePath] + size
        : size;
    });
  });

  return directoryDB;
};

export const partOne = (input: string) => {
  const fileSizeMap = getFileSizeMap(USE_EXAMPLE ? example : input);

  console.log({ fileSizeMap });
  const directorySizeMap = getDirectorySizeMap(fileSizeMap);
  console.log({ directorySizeMap });

  return Object.values(directorySizeMap).reduce(
    (acc, size) => (size <= MAX_SIZE ? acc + size : acc),
    0
  );
};
// export const partTwo = (input: string) => input.split("\n");
