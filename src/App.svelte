<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { writable } from "svelte/store";

  export let name: string;

  const currentBoard = writable<[[]]>();

  const updateBoard = (value: string): [[]] => {
    const board: [[]] = JSON.parse(value);
    currentBoard.set(board);
    return board;
  };

  let boardPromise = invoke("generate_board").then(updateBoard);

  const onClickCell = (row: number, column: number) => {
    if (!$currentBoard[row][column]) {
      invoke("solve_value", {
        board: $currentBoard,
        row,
        column,
      })
        .then(updateBoard)
        .catch((error) => {
          boardPromise = Promise.reject(error);
        });
    }
  };

  const cellStyle = (row: number, column: number): string => {
    let styles = [];

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
</script>

<main>
  <h1>Hello {name}!</h1>
  <p>
    Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn
    how to build Svelte apps.
  </p>
  {#await boardPromise}
    <span>Generating the board...</span>
  {:then board}
    <table>
      {#each $currentBoard as row, rowNumber}
        <tr>
          {#each row as cell, columnNumber}
            <td
              on:click={() => onClickCell(rowNumber, columnNumber)}
              class={cellStyle(rowNumber, columnNumber)}>{cell || " "}</td
            >
          {/each}
        </tr>
      {/each}
    </table>
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
  }

  td {
    border-style: inset;
    width: 2em;
    height: 2em;
  }

  td.solidTop {
    border-top: solid;
  }

  td.solidBottom {
    border-bottom: solid;
  }

  td.solidLeft {
    border-left: solid;
  }

  td.solidRight {
    border-right: solid;
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
