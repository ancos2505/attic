import { console_log } from "./DevDebugger";

export function getSystemTheme() {
    if (
        window.matchMedia &&
        window.matchMedia("(prefers-color-scheme: dark)").matches
    ) {
        return "dark";
    } else {
        return "light";
    }
}

export function setThemefromLocalStorage() {
    console_log("setThemefromLocalStorage()")
    console_log('XPTO Running in development mode')
}