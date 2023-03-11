<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { fileContent } from "./stores";
    
    let editorText = "";
    fileContent.subscribe((text) => {
        editorText = text;
    });

    let name: string = "";
    let greetMsg: string = "";

</script>

<main class="main-class">
    <input type="text" name="" id="" bind:value={name} />
    <button
        on:click={async () => {
            greetMsg = await invoke("greet", { name });
        }}>Click here</button
    >
    <button
        on:click={() => {
            name = "";
            greetMsg = "";
        }}>Clear</button
    >
    <span>{greetMsg}</span>
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
