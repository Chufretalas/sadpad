<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    
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
