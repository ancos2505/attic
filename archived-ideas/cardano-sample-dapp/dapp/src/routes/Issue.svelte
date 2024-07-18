<script>
    import svelteLogo from "../assets/svelte.svg";
    import Counter from "../lib/Counter.svelte";
    import ThemeSelector from "../lib/ThemeSelector.svelte";
    import {
        getSystemTheme,
        setThemefromLocalStorage,
    } from "../lib/ThemeDetection";
    import { console_log, console_warn } from "../lib/DevDebugger";

    let { route = $bindable() } = $props();

    const theRouteStr = route.toString() || "";
    console_log("theRouteStr", theRouteStr);
    // const args = currentRoute.split("/", 3);
    const args = theRouteStr.split("/");
    console_log("args", args);
    const arg = args[2] || null;
    console_warn("arg", arg);
    let issue = $derived(arg);
    console_warn("issue", issue);
    setThemefromLocalStorage();
</script>

<div class="hero min-h-screen bg-base-200">
    <div class="hero-content text-center">
        <div class="max-w-md">
            <div class="flex justify-center">
                <img
                    class="my-6"
                    src={svelteLogo}
                    width="128"
                    height="128"
                    alt="the Fresh logo: a sliced lemon dripping with juice"
                />
            </div>
            <h1 class="text-5xl font-bold">Issue: {issue}</h1>
            <p class="py-6">
                Provident cupiditate voluptatem et in. Quaerat fugiat ut
                assumenda excepturi exercitationem quasi. In deleniti eaque aut
                repudiandae et a id nisi.
            </p>

            <Counter />
            <ThemeSelector initialTheme={getSystemTheme()} />
        </div>
    </div>
</div>
