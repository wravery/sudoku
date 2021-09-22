<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { current } from "./store";
  import Board from "./Board.svelte";

  let boardPromise = invoke("generate_board").then((value: string) => {
    const board: number[][] = JSON.parse(value);
    current.set(board);
  });

  const onClickCell = (event: CustomEvent<{ row: number; column: number }>) => {
    const { row, column } = event.detail;
    if (!$current[row][column]) {
      invoke("solve_value", {
        board: $current,
        row,
        column,
      })
        .then((value: string) => {
          current.update((board) => {
            board[row][column] = JSON.parse(value);
            return board;
          });
        })
        .catch((error) => {
          boardPromise = Promise.reject(error);
        });
    }
  };
</script>

<main>
  <h1>Sudoku!</h1>
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

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
