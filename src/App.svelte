<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { keyboardHandler } from "./keyboard";
  import { current, onNewGame } from "./store";
  import Board from "./Board.svelte";
  import Options from "./Options.svelte";

  let boardPromise: Promise<void>;

  const onClickCell = (
    event: CustomEvent<{
      row: number;
      column: number;
    }>
  ) => {
    const { row, column } = event.detail;
    if (!$current[row][column]) {
      invoke("solve_value", {
        board: $current,
        row,
        column,
      })
        .then((value) => {
          current.update((board) => {
            board[row][column] = value as number;
            return board;
          });
        })
        .catch((error) => {
          boardPromise = Promise.reject(error);
        });
    }
  };

  const onReload = () => {
    document.addEventListener("contextmenu", function (event) {
      event.preventDefault();
      return false;
    });

    onNewGame();
    boardPromise = invoke("generate_board").then((value) => {
      current.set(value as number[][]);
    });
  };

  onMount(onReload);
</script>

<svelte:window on:keydown={keyboardHandler} />

<main>
  <h1>Sudoku!</h1>
  <Options on:reload={onReload} />
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
