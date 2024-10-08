import { invoke } from "@tauri-apps/api/tauri";
import {
  current,
  selected,
  cells,
  takingNotes,
  showHints,
  snapshots,
} from "./store";

let $selected: { row: number; column: number } | null = null;
let $cells: (HTMLDivElement | null)[][] | null = null;
let $takingNotes = false;

const focusSelected = () => {
  if (!$selected || !$cells) {
    return;
  }

  const { row, column } = $selected;
  const cell = $cells[row][column];
  if (cell) {
    cell.focus();
  }
};

selected.subscribe((value) => {
  $selected = value;
  focusSelected();
});
cells.subscribe((value) => {
  $cells = value;
  focusSelected();
});
takingNotes.subscribe((value) => {
  $takingNotes = value;
});

export const keyboardHandler = (e: KeyboardEvent) => {
  let handled = false;

  switch (e.code) {
    case "KeyH": {
      showHints.update((value) => !value);
      handled = true;
      break;
    }

    case "KeyN": {
      takingNotes.update((value) => !value);
      handled = true;
      break;
    }

    case "KeyC": {
      snapshots.update((values) => {
        let latest = JSON.parse(JSON.stringify(values[0]));
        values.unshift(latest);
        return values;
      });
      handled = true;
      break;
    }

    case "KeyV": {
      snapshots.update((values) => {
        if (values.length > 1) {
          values.shift();
        }
        return values;
      });
      handled = true;
      break;
    }
  }

  if (!$selected || !$cells) {
    return;
  }

  let updateSelection = false;
  let { row, column } = $selected;

  if (!handled) {
    const matches = e.code.match(/^(?:Arrow(Left|Right|Up|Down)|(?:Key(W|A|S|D)))/);
    if (matches?.length === 3) {
      switch (matches[1] || matches[2]) {
        case "A":
        case "Left": {
          updateSelection = --column >= 0;
          handled = true;
          break;
        }

        case "D":
        case "Right": {
          updateSelection = ++column < 9;
          handled = true;
          break;
        }

        case "W":
        case "Up": {
          updateSelection = --row >= 0;
          handled = true;
          break;
        }

        case "S":
        case "Down": {
          updateSelection = ++row < 9;
          handled = true;
          break;
        }
      }
    }
  }

  if (!handled) {
    const matches = e.code.match(/^(?:Digit|Numpad)([0-9])$/);
    if (matches?.length === 2) {
      const digit = parseInt(matches[1]);
      if (digit >= 0 && digit <= 9) {
        if ($takingNotes) {
          snapshots.update((values) => {
            if (digit === 0) {
              values[0][row][column] = [
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0],
              ];
            } else {
              const noteRow = ~~((digit - 1) / 3);
              const noteColumn = (digit - 1) % 3;
              if (values[0][row][column][noteRow][noteColumn]) {
                values[0][row][column][noteRow][noteColumn] = 0;
              } else {
                values[0][row][column][noteRow][noteColumn] = digit;
              }
            }
            return values;
          });
        } else {
          current.update((board) => {
            if (digit === 0 || board[row][column].value === 0) {
              board[row][column].value = digit;
              board[row][column].isWrong = false;
              if (digit !== 0) {
                invoke("solve_value", {
                  board: board.map((row) => row.map((cell) => cell.value)),
                  row,
                  column,
                })
                  .then(() => {
                    snapshots.update((values) => {
                      values[0][row][column] = [
                        [0, 0, 0],
                        [0, 0, 0],
                        [0, 0, 0],
                      ];
                      const sectionRow = row - (row % 3);
                      const sectionColumn = column - (column % 3);
                      const noteRow = ~~((digit - 1) / 3);
                      const noteColumn = (digit - 1) % 3;
                      for (let i = 0; i < 9; ++i) {
                        values[0][i][column][noteRow][noteColumn] = 0;
                        values[0][row][i][noteRow][noteColumn] = 0;
                        const rowOffset = ~~(i / 3);
                        const columnOffset = i % 3;
                        values[0][sectionRow + rowOffset][sectionColumn + columnOffset][noteRow][noteColumn] = 0;
                      }
                      return values;
                    });
                  })
                  .catch(() => {
                    current.update((board) => {
                      board[row][column].isWrong = true;
                      return board;
                    })
                  });
              }
            }
            return board;
          });
        }
      }
      handled = true;
    }
  }

  if (!handled) {
    return;
  }

  if (updateSelection) {
    selected.set({ row, column });
  }

  e.preventDefault();
};
