<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabListState, getTabState } from './context.svelte.ts';

	let {
		value,
		disabled = false,
		children
	}: {
		value: string;
		disabled?: boolean;
		children: Snippet;
	} = $props();

	let data = getTabListState();
	let tState = getTabState(data.id);
</script>

{#if !disabled && value === tState.currentTab}
	<div class={tState.classes?.content}>
		{@render children()}
	</div>
{/if}
