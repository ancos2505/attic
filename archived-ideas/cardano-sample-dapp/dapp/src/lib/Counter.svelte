<script>
	function createCounter() {
		let count = $state(0);

		function increment() {
			count += 1;
		}

		return {
			get count() { return count },
			increment
		};
	}

	const counter = createCounter();
</script>

<button class="btn btn-primary" onclick={counter.increment}>
	clicks: {counter.count}
</button>