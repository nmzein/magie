<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabState } from './context.svelte.ts';

	let {
		value,
		disabled = false,
		children
	}: {
		value: string;
		disabled?: boolean;
		children: Snippet;
	} = $props();

	let tState = getTabState();
</script>

{#if !disabled && value === tState.currentTab}
	<div class={tState.classes.content}>
		{@render children()}
	</div>
{/if}
