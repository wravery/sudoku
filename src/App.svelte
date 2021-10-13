<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { keyboardHandler } from "./keyboard";
  import { current, takingNotes, showHints } from "./store";
  import Board from "./Board.svelte";

  let boardPromise = invoke("generate_board").then((value: number[][]) => {
    current.set(value);
  });

  const onClickCell = (event: CustomEvent<{ row: number; column: number }>) => {
    const { row, column } = event.detail;
    if (!$current[row][column]) {
      invoke("solve_value", {
        board: $current,
        row,
        column,
      })
        .then((value: number) => {
          current.update((board) => {
            board[row][column] = value;
            return board;
          });
        })
        .catch((error) => {
          boardPromise = Promise.reject(error);
        });
    }
  };
</script>

<svelte:window on:keydown={keyboardHandler} />

<main>
  <h1>Sudoku!</h1>
  <div class="options">
    <input
      type="checkbox"
      bind:checked={$takingNotes}
      id="takingNotesCheckbox"
      tabindex="-1"
    />
    <label for="takingNotesCheckBox"
      ><span class="hotkey">N</span>ote Taking</label
    >
    <input
      type="checkbox"
      bind:checked={$showHints}
      id="showHintCheckbox"
      tabindex="-1"
    />
    <label for="showHintCheckbox">Show <span class="hotkey">H</span>ints</label>
  </div>
  {#await boardPromise}
    <span>Generating the board...</span>
  {:then}
    <Board on:clickCell={onClickCell} />
  {:catch error}
    <span>Error generating the board: {error}</span>
  {/await}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
    overflow-y: hidden;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  div.options {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
  }

  span.hotkey {
    text-decoration: underline;
  }

  input {
    border: unset;
    padding: unset;
    margin: 0.5em;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
