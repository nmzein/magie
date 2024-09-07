<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabState } from './context.svelte.ts';

	let { children }: { children: Snippet } = $props();

	let tState = getTabState();
</script>

<div class={tState.classes.list}>
	{@render children()}
</div>
