<script>
  import Home from "./routes/Home.svelte";
  import Issue from "./routes/Issue.svelte";
  import { console_log, console_warn } from "./lib/DevDebugger";
  import Menu from "./lib/Menu/Menu.svelte";

  const container = document.querySelector("div#app");

  const initialRoute = window.location.hash.split("#", 2).pop() || "/";
  console_log("initialRoute", initialRoute);

  let storedRoute = $state(initialRoute);

  window.addEventListener("popstate", function (event) {
    const parsedRoute = window.location.hash.split("#", 2).pop() || "/";
    storedRoute = parsedRoute;
    console_log("storedRoute", storedRoute);
  });
</script>

<div class="flex h-screen">
  {#key storedRoute}
    <div class="flex-initial w-64">
      <Menu route={storedRoute} />
    </div>

    <div class="flex-auto">
      {#if storedRoute.startsWith("/issue")}
        <Issue route={storedRoute} />
      {:else}
        <Home />
      {/if}
    </div>
  {/key}
</div>
