<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { fileContent } from "../routes/stores";

    export let changed: () => void;
</script>

<main class="main-class">
    <textarea
        name=""
        id=""
        cols="30"
        rows="10"
        bind:value={$fileContent}
        on:keypress={changed}
    />
</main>

<style>
    .main-class {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        row-gap: 1rem;
        font-size: 2rem;
    }

    textarea {
        width: 90vw;
        height: 90vh;
        resize: none;
    }
</style>
