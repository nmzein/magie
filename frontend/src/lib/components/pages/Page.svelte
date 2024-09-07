<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getPagesState } from './context.svelte.ts';
	import Nav from './Nav.svelte';
	import Back from './Back.svelte';
	import Next from './Next.svelte';
	import Done from './Done.svelte';
	import { defined } from '$helpers';

	let {
		nextDisabled,
		done,
		children
	}: { nextDisabled: boolean; done?: () => void; children: Snippet } = $props();

	let pState = getPagesState();

	const id = pState.registerPage();
</script>

{#if pState.currentPage === id}
	{@render children()}

	<Nav>
		<Back>Back</Back>
		<Next disabled={nextDisabled}>Next</Next>
		{#if defined(done)}
			<Done enabled={!nextDisabled && pState.lastPage} {done}>Done</Done>
		{/if}
	</Nav>
{/if}
