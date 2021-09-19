<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let name: string;

  let boardPromise = invoke("generate_board").then(
    (value: string) => JSON.parse(value) as [[]]
  );
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
      {#each board as row}
        <tr>
          {#each row as cell}
            <td>{cell || " "}</td>
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
	  width: 1em;
	  height: 1em;
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
