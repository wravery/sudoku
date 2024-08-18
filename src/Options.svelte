<script lang="ts">
  import { takingNotes, showHints } from "./store";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ reload?: never }>();
  const reload = () => dispatch("reload");
</script>

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
  <button on:click={reload}>New Game</button>
</div>

<style>
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

  button {
    margin: 0 0 0 0.5em;
    border-radius: 4px;
    background-color: tomato;
    color: white;
  }
</style>
