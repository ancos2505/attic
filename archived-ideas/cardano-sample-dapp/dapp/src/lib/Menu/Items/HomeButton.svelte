<script>
    import { console_log, console_warn } from "../../DevDebugger";
    import { navigateTo } from "../../Router";
    let { currentPage = $bindable() } = $props();
    const bindRoute = "";

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
                d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
            /></svg
        >
        Home
    </button>
</li>
