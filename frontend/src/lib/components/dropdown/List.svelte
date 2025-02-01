<script lang="ts">
	import type { Snippet } from 'svelte';
	import { context } from './context.svelte';
	import { twMerge } from 'tailwind-merge';
	import { onClickOutside } from 'runed';

	let { children }: { children: Snippet } = $props();

	let dropdown: HTMLDivElement | undefined = $state();
	const ctx = context.get();

	onClickOutside(
		() => dropdown,
		() => ctx.close()
	);
</script>

{#if ctx.show}
	<div class={twMerge('select-none', ctx.classes.list)} bind:this={dropdown}>
		{@render children()}
	</div>
{/if}
