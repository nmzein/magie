<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div>
	{@render children()}
</div>

<style lang="scss">
	div {
		display: flex;
		justify-content: space-between;
		width: 100%;
	}
</style>
