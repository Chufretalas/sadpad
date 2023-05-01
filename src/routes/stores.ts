import { writable } from "svelte/store";


export const filePath = writable("new file")
export const fileContent = writable("")