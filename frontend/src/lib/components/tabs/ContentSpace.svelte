<script lang="ts">
	import type { Snippet } from 'svelte';
	import { setTabListState } from './context.svelte.js';
	import type { Bounds } from '$types';

	let {
		id,
		bounds = $bindable(),
		children
	}: { id: string; bounds: Bounds | undefined; children: Snippet } = $props();

	setTabListState(id);

	let containerBounds: Bounds | undefined = $state();
	$effect(() => {
		bounds = containerBounds;
	});
</script>

<div
	class="h-screen w-full flex-1 shrink-0 overflow-hidden p-[10px] pr-0"
	bind:contentRect={containerBounds}
>
	{@render children()}
</div>
