<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabState } from './context.svelte.ts';

	let {
		value,
		disabled = false,
		children
	}: {
		value: string;
		disabled: boolean;
		children: Snippet;
	} = $props();

	let state = getTabState();
</script>

{#if !disabled && value === state.currentTab}
	<div class={state.classes.content}>
		{@render children()}
	</div>
{/if}
