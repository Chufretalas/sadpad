<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { filePath, fileContent } from "./stores";
    import TextBox from "../components/TextBox.svelte";

    let saved = true;

    async function openNew() {
        $filePath = "new file";
        $fileContent = "";
    }

    async function openFile() {
        const openedFile: string[] = await invoke("open_file");
        if (openedFile[0] !== "") {
            filePath.set(openedFile[0]);
            fileContent.set(openedFile[1]);
        }
    }

    async function saveFile() {
        const savedPath = await invoke("save_file", {
            pathStr: $filePath.replaceAll('"', "").replaceAll("\\\\", "\\"),
            content: $fileContent,
        });
        if (savedPath !== "") {
            $filePath = savedPath as string;
            saved = true;
        }
    }
</script>

<header>
    <section class="menu-buttons">
        <button class="menu-button" on:click={openNew}>New file</button>
        <button class="menu-button" on:click={openFile}>Open file</button>
        <button class="menu-button" on:click={saveFile}>Save</button>
    </section>
    <h2 class="file-name">{$filePath} {saved ? "" : "*"}</h2>
</header>
<TextBox
    changed={() => {
        // console.log("changed")
        saved = false;
    }}
/>

<svelte:window
    on:keydown={(e) => {
        if (e.key === "s" && e.ctrlKey === true) {
            saveFile();
        }
    }}
/>

<style>
    header {
        background-color: #344955;
        box-shadow: 0 3px 10px 2px #182228;
        margin-bottom: 10px;
        padding: 1px;
        display: grid;
        grid-template-columns: 1fr 3fr 1fr;
        grid-template-rows: 1fr;
    }
    .menu-buttons {
        display: flex;
        gap: 0.2rem;
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

    .file-name {
        justify-self: center;
    }
</style>
