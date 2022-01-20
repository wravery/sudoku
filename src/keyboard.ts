import {
  current,
  selected,
  cells,
  takingNotes,
  showHints,
  snapshots,
} from "./store";

let $selected: { row: number; column: number } = null;
let $cells: HTMLDivElement[][] = null;
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

    case "KeyS": {
      snapshots.update((values) => {
        let latest = JSON.parse(JSON.stringify(values[0]));
        values.unshift(latest);
        return values;
      });
      handled = true;
      break;
    }

    case "KeyR": {
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
    const matches = e.code.match(/^Arrow(Left|Right|Up|Down)/);
    if (matches?.length === 2) {
      switch (matches[1]) {
        case "Left": {
          updateSelection = --column >= 0;
          handled = true;
          break;
        }

        case "Right": {
          updateSelection = ++column < 9;
          handled = true;
          break;
        }

        case "Up": {
          updateSelection = --row >= 0;
          handled = true;
          break;
        }

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
            if (digit === 0 || board[row][column] === 0) {
              board[row][column] = digit;
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
