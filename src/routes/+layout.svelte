<script lang="ts">
    import "./reset.css"
    import "./styles.css"

    import { invoke } from "@tauri-apps/api/tauri";

    async function openFile() {
        const path = await invoke("open_file")
        console.log(path)
    }

</script>

<header>
    <button class="menu-button" on:click={openFile}>Open file</button>
</header>
<slot/>

<style>
    header {
        background-color: #344955;
        box-shadow: 0 3px 10px 2px #182228;
        margin-bottom: 10px;
        padding: 1px;
    }
    .menu-button {
        border: none;
        border-radius: 5px;
        width: 5rem;
        font: bolder;
        background-color: white;
        transition: all 100ms;
    }

    .menu-button:hover {
        background-color: gray;
        cursor: pointer;
    }

    .menu-button:active {
        transform: scale(0.95);
        background-color: rgb(100, 100, 100);
    }
</style>