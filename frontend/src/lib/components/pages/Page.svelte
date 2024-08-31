<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getPagesState } from './context.svelte.ts';
	import Nav from './Nav.svelte';
	import Back from './Back.svelte';
	import Next from './Next.svelte';

	let { nextDisabled, children }: { nextDisabled: boolean; children: Snippet } = $props();

	let pState = getPagesState();

	const id = pState.registerPage();
</script>

{#if pState.currentPage === id}
	{@render children()}

	<Nav>
		<Back>Back</Back>
		<Next disabled={nextDisabled}>Next</Next>
	</Nav>
{/if}
