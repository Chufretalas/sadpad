<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { fileContent } from "./stores";

</script>

<main class="main-class">
    <textarea name="" id="" cols="30" rows="10" bind:value={$fileContent} />
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
        width: 90%;
        height: 90%;
    }
</style>
