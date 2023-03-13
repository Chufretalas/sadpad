import { writable } from "svelte/store";


export const filePath = writable("no file")
export const fileContent = writable("")