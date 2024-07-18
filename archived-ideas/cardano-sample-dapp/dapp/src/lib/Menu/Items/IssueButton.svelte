<script>
    import { console_log, console_warn } from "../../DevDebugger";
    import { navigateTo } from "../../Router";
    let { currentPage = $bindable() } = $props();
    const bindRoute = "issue";

    const currentPageStr = currentPage.toString() || "";
    console_log("currentPageStr", currentPageStr);
    const activeClass = currentPageStr === bindRoute ? "active" : "";
</script>

<li>
    <button class={activeClass} onclick={() => navigateTo(bindRoute)}>
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-5 w-5"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            /></svg
        >
        Issue
    </button>
</li>
