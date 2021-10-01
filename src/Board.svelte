<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { current } from "./store";
  import { derived, writable } from "svelte/store";
  import { fly } from "svelte/transition";
  import { createEventDispatcher } from "svelte";

  const dispatch =
    createEventDispatcher<{ clickCell: { row: number; column: number } }>();

  const computeStyle = (row: number, column: number): string => {
    let styles = ["cell"];

    switch (row % 3) {
      case 0:
        styles.push("solidTop");
        break;
      case 2:
        styles.push("solidBottom");
        break;
    }

    switch (column % 3) {
      case 0:
        styles.push("solidLeft");
        break;
      case 2:
        styles.push("solidRight");
        break;
    }

    return styles.join(" ");
  };

  const selected = writable<{ row: number; column: number } | null>();
  const remainingValues = derived<typeof selected, Promise<number[]> | null>(
    selected,
    ($selected) => {
      if (!$selected) {
        return null;
      }
      return invoke("get_possible_values", {
        board: $current,
        row: $selected.row,
        column: $selected.column,
      }).then((values: string) => JSON.parse(values));
    }
  );
  let cells: HTMLDivElement[][] = [];

  for (let row = 0; row < 9; ++row) {
    let rowCells = [];
    for (let column = 0; column < 9; ++column) {
      rowCells.push(null);
    }
    cells.push(rowCells);
  }

  const onFocusCell = (row: number, column: number) => {
    if (!$current[row][column]) {
      selected.set({ row, column });
    }
  };

  const onClickCell = (target: EventTarget, row: number, column: number) => {
    (target as HTMLDivElement).blur();
    if (!$current[row][column]) {
      selected.set(null);
      dispatch("clickCell", { row, column });
    }
  };

  const onKeyDown = (e: KeyboardEvent) => {
    if (!$selected) {
      return;
    }

    let handled = false;
    let updateSelection = false;
    let { row, column } = $selected;

    switch (e.code) {
      case "Space": {
        const cell = cells[row][column];
        if (cell) {
          onClickCell(cell, row, column);
          handled = true;
        }
        break;
      }

      case "ArrowLeft": {
        while (--column >= 0) {
          if (!$current[row][column]) {
            handled = updateSelection = true;
            break;
          }
        }
        break;
      }

      case "ArrowRight": {
        while (++column < 9) {
          if (!$current[row][column]) {
            handled = updateSelection = true;
            break;
          }
        }
        break;
      }

      case "ArrowUp": {
        while (--row >= 0) {
          if (!$current[row][column]) {
            handled = updateSelection = true;
            break;
          }
        }
        break;
      }

      case "ArrowDown": {
        while (++row < 9) {
          if (!$current[row][column]) {
            handled = updateSelection = true;
            break;
          }
        }
        break;
      }
    }

    if (!handled) {
      const matches = e.code.match(/^Digit([0-9])$/);
      if (matches?.length === 2) {
        const digit = parseInt(matches[1]);
        if (digit >= 0 && digit <= 9) {
          current.update((board) => {
            board[row][column] = digit;
            return board;
          });
        }
        handled = true;
      }
    }

    if (!handled) {
      return;
    }

    if (updateSelection) {
      selected.set({ row, column });

      const cell = cells[row][column];
      if (cell) {
        cell.focus();
      }
    }

    e.preventDefault();
  };
</script>

<section transition:fly={{ y: 500 }} on:keydown={onKeyDown}>
  {#each $current as rowCells, rowNumber (rowNumber)}
    <div class="row">
      {#each rowCells as cell, columnNumber (rowNumber * 9 + columnNumber)}
        <div
          bind:this={cells[rowNumber][columnNumber]}
          class={computeStyle(rowNumber, columnNumber)}
          class:selected={$selected &&
            $selected.row === rowNumber &&
            $selected.column == columnNumber}
          on:click={(e) => onClickCell(e.target, rowNumber, columnNumber)}
          on:focus={() => onFocusCell(rowNumber, columnNumber)}
          tabindex={cell ? -1 : 0}
        >
          {#if cell}
            <span transition:fly={{ y: -10 }}>{cell}</span>
          {/if}
        </div>
      {/each}
    </div>
  {/each}
  {#await $remainingValues then values}
    {#if values}
      <div class="hint">{`Hint: ${values.join(", ")}`}</div>
    {/if}
  {/await}
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-around;
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    cursor: default;
  }

  div.hint {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-around;
    font-style: italic;
  }

  div.row {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-around;
  }

  div.cell {
    display: flex;
    width: 2em;
    height: 2em;
    align-items: center;
    justify-content: center;
    border: solid 1px;
  }

  div.cell:focus {
    outline: none;
  }

  div.cell:hover:empty {
    background-color: beige;
  }

  div.cell:active:empty {
    background-color: bisque;
  }

  div.cell.selected {
    background-color: azure;
  }

  div.solidTop {
    border-top-width: 2px;
  }

  div.solidBottom {
    border-bottom-width: 2px;
  }

  div.solidLeft {
    border-left-width: 2px;
  }

  div.solidRight {
    border-right-width: 2px;
  }
</style>
