<script lang="ts">
	import { explorer } from '$states';
	import type { Route } from '$types';
	import type { Snippet } from 'svelte';

	let { route, children }: { route: Route; children: Snippet } = $props();
</script>

<button onclick={() => explorer.routeTo(route)}>
	{@render children()}
</button>

<style lang="scss">
	button {
		width: 100%;
		gap: 10px;
		display: flex;
		align-items: center;
		text-align: left;
		padding: 7.5px 10px 7.5px 10px;
		border-radius: 7px;

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}
</style>
