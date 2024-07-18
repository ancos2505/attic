<script>
    import {
        getSystemTheme,
        setThemefromLocalStorage,
    } from "../ThemeDetection";
    import { console_log, console_warn } from "../DevDebugger";
    import HomeButton from "./Items/HomeButton.svelte";
    import IssueButton from "./Items/IssueButton.svelte";
    import StatsButton from "./Items/StatsButton.svelte";

    let { route = $bindable() } = $props();

    const theRouteStr = route.toString() || "";
    console_log("theRouteStr", theRouteStr);
    // const args = currentRoute.split("/", 3);
    const args = theRouteStr.split("/");
    console_log("args", args);
    const currentPage = args[1] || "";
    console_warn("currentPage", currentPage);
    const arg = args[2] || null;
    console_warn("arg", arg);
    let issue = $derived(arg);
    console_warn("issue", issue);
    setThemefromLocalStorage();
</script>

<div class="flex justify-center p-4">
    <ul class="menu bg-base-200 w-56 rounded-box">
        <HomeButton {currentPage} />
        <IssueButton {currentPage} />
        <StatsButton {currentPage} />
    </ul>
</div>
