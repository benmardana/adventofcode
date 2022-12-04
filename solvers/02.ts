enum SCORE {
  ROCK = 1,
  PAPER = 2,
  SCISSORS = 3,

  WIN = 6,
  LOSS = 0,
  DRAW = 3,
}

enum CHOICE {
  OPPONENT_ROCK = "A",
  OPPONENT_PAPER = "B",
  OPPONENT_SCISSORS = "C",

  PLAYER_ROCK = "X",
  PLAYER_PAPER = "Y",
  PLAYER_SCISSORS = "Z",
}

const scoreMap: Record<string, number> = {
  AX: SCORE.ROCK + SCORE.DRAW,
  AY: SCORE.PAPER + SCORE.WIN,
  AZ: SCORE.SCISSORS + SCORE.LOSS,

  BX: SCORE.ROCK + SCORE.LOSS,
  BY: SCORE.PAPER + SCORE.DRAW,
  BZ: SCORE.SCISSORS + SCORE.WIN,

  CX: SCORE.ROCK + SCORE.WIN,
  CY: SCORE.PAPER + SCORE.LOSS,
  CZ: SCORE.SCISSORS + SCORE.DRAW,
};

const resultMap: Record<string, (opponent: string) => number> = {
  X: (opponent: string) => {
    if (opponent === CHOICE.OPPONENT_ROCK) {
      return scoreMap[`${opponent}${CHOICE.PLAYER_SCISSORS}`];
    }
    if (opponent === CHOICE.OPPONENT_PAPER) {
      return scoreMap[`${opponent}${CHOICE.PLAYER_ROCK}`];
    }
    return scoreMap[`${opponent}${CHOICE.PLAYER_PAPER}`];
  },
  Y: (opponent: string) => {
    if (opponent === CHOICE.OPPONENT_ROCK) {
      return scoreMap[`${opponent}${CHOICE.PLAYER_ROCK}`];
    }
    if (opponent === CHOICE.OPPONENT_PAPER) {
      return scoreMap[`${opponent}${CHOICE.PLAYER_PAPER}`];
    }
    return scoreMap[`${opponent}${CHOICE.PLAYER_SCISSORS}`];
  },
  Z: (opponent: string) => {
    if (opponent === CHOICE.OPPONENT_ROCK) {
      return scoreMap[`${opponent}${CHOICE.PLAYER_PAPER}`];
    }
    if (opponent === CHOICE.OPPONENT_PAPER) {
      return scoreMap[`${opponent}${CHOICE.PLAYER_SCISSORS}`];
    }
    return scoreMap[`${opponent}${CHOICE.PLAYER_ROCK}`];
  },
};

export const partOne = (input: string) =>
  input
    ?.split("\n")
    .map((line) => scoreMap[line.replace(" ", "")])
    .reduce((acc, curr) => acc + curr, 0);

export const partTwo = (input: string) =>
  input
    ?.split("\n")
    .map((line) => {
      const [opponent, result] = line.split(" ");
      return resultMap[result](opponent);
    })
    .reduce((acc, curr) => acc + curr, 0);
