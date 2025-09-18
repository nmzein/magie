<script lang="ts">
	import type { Snippet } from 'svelte';
	import { context } from './context.svelte';
	import { twMerge } from 'tailwind-merge';

	let { children }: { children: Snippet } = $props();

	const ctx = context.get();
</script>

{#if ctx.show}
	<div class={twMerge('select-none', ctx.classes.list)} bind:this={ctx.listElement}>
		{@render children()}
	</div>
{/if}
