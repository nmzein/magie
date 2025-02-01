<script lang="ts">
	import type { Snippet } from 'svelte';
	import { context } from './context.svelte.ts';
	import { defined } from '$helpers';
	import Nav from './Nav.svelte';
	import Back from './Back.svelte';
	import Next from './Next.svelte';
	import Done from './Done.svelte';

	let {
		nextDisabled,
		done,
		children
	}: { nextDisabled: boolean; done?: () => void; children: Snippet } = $props();

	const ctx = context.get();
	const id = ctx.registerPage();
</script>

{#if ctx.currentPage === id}
	{@render children()}

	<Nav>
		<Back>Back</Back>
		<Next disabled={nextDisabled}>Next</Next>
		{#if defined(done)}
			<Done enabled={!nextDisabled && ctx.lastPage} {done}>Done</Done>
		{/if}
	</Nav>
{/if}
