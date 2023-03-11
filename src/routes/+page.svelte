<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { fileContent } from "./stores";

    let editorText = "";
    fileContent.subscribe((text) => {
        editorText = text;
    });
    
</script>

<main class="main-class">
    <textarea name="" id="" cols="30" rows="10" bind:value={editorText} />
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
</style>
