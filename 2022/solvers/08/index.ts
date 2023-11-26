const example = `30373
25512
65332
33549
35390`;
export const partOne = (input: string) => {
  const rows = input.split("\n").map((row) => row.split("").map(Number));
  const columns = transpose(rows);

  const horizontallyVisibleIndices = rows.map(findVisibleTreeIndices);
  const verticallyVisibleIndices = columns.map(findVisibleTreeIndices);

  const visibleTrees: Record<string, boolean> = {};

  horizontallyVisibleIndices.forEach((row, rowIndex) => {
    row.forEach((colIndex) => {
      visibleTrees[`${rowIndex}-${colIndex}`] = true;
    });
  });
  verticallyVisibleIndices.forEach((col, colIndex) => {
    col.forEach((rowIndex) => {
      visibleTrees[`${rowIndex}-${colIndex}`] = true;
    });
  });
  return Object.keys(visibleTrees).length;
};

export const partTwo = (input: string) => {
  const rows = input.split("\n").map((row) => row.split("").map(Number));

  const scores = rows.map((row, rowIndex) => {
    return row.map((tree, colIndex) => {
      let rightScore = 0;
      let leftScore = 0;
      let upScore = 0;
      let downScore = 0;

      // right
      for (let index = colIndex + 1; index < row.length; index++) {
        rightScore += 1;
        if (row[index] >= tree) break;
      }

      // left
      for (let index = colIndex - 1; index >= 0; index--) {
        leftScore += 1;
        if (row[index] >= tree) break;
      }

      // down
      for (let index = rowIndex + 1; index < rows.length; index++) {
        downScore += 1;
        if (rows[index][colIndex] >= tree) break;
      }

      // up
      for (let index = rowIndex - 1; index >= 0; index--) {
        upScore += 1;
        if (rows[index][colIndex] >= tree) break;
      }

      return rightScore * leftScore * upScore * downScore;
    });
  });

  return scores.reduce((acc, row) => {
    const rowMax = row.reduce((acc, score) => (score > acc ? score : acc));
    return rowMax > acc ? rowMax : acc;
  }, 0);
};

const transpose = <T>(matrix: T[][]) =>
  matrix[0].map((_, colIndex) => matrix.map((row) => row[colIndex]));

const findVisibleTreeIndices = (trees: number[]) => {
  let maxTreeHeight = -Infinity;
  const front: number[] = [];
  trees.forEach((treeHeight, index) => {
    if (treeHeight > maxTreeHeight) front.push(index);
    maxTreeHeight = Math.max(maxTreeHeight, treeHeight);
  });

  const peak = front.at(-1)! + 1;
  maxTreeHeight = -Infinity;

  const back: number[] = [];
  for (let index = trees.length - 1; index >= peak; index--) {
    const treeHeight = trees[index];
    if (treeHeight > maxTreeHeight) back.push(index);
    maxTreeHeight = Math.max(maxTreeHeight, treeHeight);
  }

  return [...front, ...back.reverse()];
};
