<script>
    import { console_log } from "./DevDebugger";

	let { initialTheme: theInitialTheme } = $props();

	function createThemeSelector() {
		// Get Current Theme
		let theme = $state("light");

		function init(themeName) {
			themeName === "light" ? (theme = "light") : (theme = "dark");
			document.documentElement.setAttribute("data-theme", theme);

			console_log("toggle() theme", theme);
		}
		function toggle() {
			theme === "light" ? (theme = "dark") : (theme = "light");
			document.documentElement.setAttribute("data-theme", theme);

			console_log("toggle() theme", theme);
		}

		return {
			get theme() {
				return theme;
			},
			init,
			toggle,
		};
	}

	const themeSelector = createThemeSelector();
	switch (theInitialTheme) {
		case "dark":
			themeSelector.init("dark");
			break;

		case "light":
			themeSelector.init("light");
			break;

		default:
			themeSelector.init("light");
			break;
	}
</script>

<div class="flex justify-center py-4">
	<label class="cursor-pointer grid place-items-center">
		<input
			checked={themeSelector.theme === "dark"}
			onclick={themeSelector.toggle}
			type="checkbox"
			value="synthwave"
			class="toggle theme-controller bg-base-content row-start-1 col-start-1 col-span-2"
		/>
		<svg
			class="col-start-1 row-start-1 stroke-base-100 fill-base-100"
			xmlns="http://www.w3.org/2000/svg"
			width="14"
			height="14"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			><circle cx="12" cy="12" r="5" /><path
				d="M12 1v2M12 21v2M4.2 4.2l1.4 1.4M18.4 18.4l1.4 1.4M1 12h2M21 12h2M4.2 19.8l1.4-1.4M18.4 5.6l1.4-1.4"
			/></svg
		>
		<svg
			class="col-start-2 row-start-1 stroke-base-100 fill-base-100"
			xmlns="http://www.w3.org/2000/svg"
			width="14"
			height="14"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"
			></path></svg
		>
	</label>
</div>
