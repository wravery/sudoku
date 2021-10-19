<script lang="ts">
  import {
    current,
    selected,
    remainingValues,
    cells,
    notes,
    showHints,
  } from "./store";
  import { fade, fly } from "svelte/transition";
  import { createEventDispatcher, tick } from "svelte";

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

  const onFocusCell = async (row: number, column: number) => {
    selected.set({ row, column });
  };

  const onClickCell = (row: number, column: number) => {
    dispatch("clickCell", { row, column });
  };
</script>

<section in:fly={{ y: 500 }}>
  {#each $current as rowCells, rowNumber (rowNumber)}
    <div class="row">
      {#each rowCells as cell, columnNumber (rowNumber * 9 + columnNumber)}
        <div
          bind:this={$cells[rowNumber][columnNumber]}
          class={computeStyle(rowNumber, columnNumber)}
          class:selected={$selected &&
            $selected.row === rowNumber &&
            $selected.column === columnNumber}
          class:empty={!cell}
          on:dblclick={() => onClickCell(rowNumber, columnNumber)}
          on:focus={() => onFocusCell(rowNumber, columnNumber)}
          tabindex={cell ? -1 : 0}
        >
          {#if cell}
            <span in:fly={{ y: -10 }}>{cell}</span>
          {:else}
            <div class="cellNotes">
              {#each $notes[rowNumber][columnNumber] as notesRow, noteRowNumber (noteRowNumber)}
                <div class="noteRow">
                  {#each notesRow as note, noteColumnNumber (noteRowNumber * 9 + noteColumnNumber)}
                    <div class="noteCell">
                      {#if note}
                        {note}
                      {/if}
                    </div>
                  {/each}
                </div>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/each}
  {#if $showHints}
    {#await $remainingValues then values}
      {#if values}
        <div class="hint" in:fade>{`Hint: ${values.join(", ")}`}</div>
      {/if}
    {/await}
  {/if}
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

  div.cell.empty:hover {
    background-color: beige;
  }

  div.cell.empty:active {
    background-color: bisque;
  }

  div.cell.selected {
    background-color: azure;
  }

  div.cellNotes {
    display: flex;
    position: relative;
    flex-direction: column;
    justify-content: center;
    width: 2em;
    height: 2em;
  }

  div.noteRow {
    display: flex;
    flex-direction: row;
    justify-content: center;
    width: 2em;
  }

  div.noteCell {
    font-size: xx-small;
    width: 1em;
    height: 1em;
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
