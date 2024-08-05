<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children, ...restProps }: { children: Snippet } = $props();
</script>

<button {...restProps}>{@render children()}</button>

<style lang="scss">
	button {
		gap: 10px;
		align-items: center;
		padding: 7.5px 10px;
		border-radius: 5px;

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}
</style>
