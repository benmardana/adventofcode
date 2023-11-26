export const PATH_DELIMITER = "\\";
export const USE_EXAMPLE = true;
export const COMMAND = "$";
export const LIST = "ls";
export const CHANGE_DIRECTORY = "cd";
export const PARENT = "..";
export const ROOT = "/";
export const DIRECTORY = "dir";
export const MAX_SIZE = 100000;
export const TOTAL_DISK_SIZE = 70000000;
export const SPACE_REQUIRED = 30000000;

export interface Directory {
  name: string;
  files: File[];
  directories: Directory[];
  size?: number;
}

interface File {
  name: string;
  size: number;
}

export const set = (obj: Directory, path: string, value: number): void => {
  const [head, ...tail] = path.split(PATH_DELIMITER);

  if (tail.length === 0) {
    obj.files.push({ name: head, size: value });
    return;
  }

  const dir = obj.directories.find(({ name }) => name === head);

  const newDir = {
    name: head,
    files: [],
    directories: [],
  };

  !dir && obj.directories.push(newDir);

  return set(dir ?? newDir, tail.join(PATH_DELIMITER), value);
};

export const calcDirectorySize = (fileSystem: Directory) => {
  fileSystem.directories.forEach(calcDirectorySize);
  fileSystem.size =
    fileSystem.files.reduce((acc, { size }) => acc + size, 0) +
    fileSystem.directories.reduce((acc, { size }) => acc + (size ?? 0), 0);
};

export const buildFileSystem = (input: string) => {
  const fileSystem: Directory = { name: "/", files: [], directories: [] };

  let path: string[] = [];

  input.split("\n").forEach((line) => {
    const [start, middle, end] = line.split(" ");
    if (start === COMMAND) {
      if (middle === LIST) {
        // handle list
        return;
      }
      if (middle === CHANGE_DIRECTORY) {
        if (end === PARENT) {
          path.pop();
          return;
        }
        if (end === ROOT) {
          // handle change to root
          path = ["/"];
          return;
        }
        // handle change to directory
        path.push(end);
        return;
      }
      console.log("unhandled command", { start, middle, end });
      return;
    }
    if (start === DIRECTORY) {
      return;
    }
    set(fileSystem, [...path, middle].join(PATH_DELIMITER), Number(start));
  });

  return fileSystem;
};

export const findDirectories = (
  fileSystem: Directory,
  deleteableDirectories: number[],
  predicate: (size: number) => boolean
) => {
  fileSystem.directories.forEach((directory) =>
    findDirectories(directory, deleteableDirectories, predicate)
  );
  if (fileSystem.size && predicate(fileSystem.size)) {
    deleteableDirectories.push(fileSystem.size);
  }
};
