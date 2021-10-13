<script lang="ts">
  import { current, takingNotes, showHints } from "./store";
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
</style>
