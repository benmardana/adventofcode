import {
  calcDirectorySize,
  findDirectories,
  buildFileSystem,
  MAX_SIZE,
  TOTAL_DISK_SIZE,
  SPACE_REQUIRED,
} from "./lib";

export const partOne = (input: string) => {
  const fileSystem = buildFileSystem(input);

  calcDirectorySize(fileSystem);

  let smallDirectories: number[] = [];
  findDirectories(
    fileSystem,
    smallDirectories,
    (size: number) => size <= MAX_SIZE
  );

  return smallDirectories.reduce((acc, size) => acc + size, 0);
};

const example = `$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k`;
export const partTwo = (input: string) => {
  const fileSystem = buildFileSystem(input);

  calcDirectorySize(fileSystem);

  const MIN_DIR_SIZE = SPACE_REQUIRED - (TOTAL_DISK_SIZE - fileSystem.size!);

  let largerDirectories: number[] = [];
  findDirectories(
    fileSystem,
    largerDirectories,
    (size: number) => size >= MIN_DIR_SIZE
  );

  return largerDirectories.sort((a, b) => a - b)[0];
};
