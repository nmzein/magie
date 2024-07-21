<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabState } from './context.svelte.ts';

	let { children }: { children: Snippet } = $props();

	let state = getTabState();
</script>

<div class={state.classes.list}>
	{@render children()}
</div>
