<script lang="ts">
	import { untrack, type Snippet } from 'svelte';
	import { getPagesState, setPagesState, type PagesClasses } from './context.svelte.ts';

	let {
		currentPage = $bindable(0),
		classes,
		children
	}: { currentPage?: number; classes?: PagesClasses; children: Snippet } = $props();

	setPagesState(currentPage, classes);

	let pState = getPagesState();

	$effect(() => {
		pState.currentPage; // Triggers the effect when pState.currentPage changes.
		untrack(() => {
			currentPage = pState.currentPage;
		});
	});
</script>

<div class="flex h-full w-full flex-col gap-5">
	{@render children()}
</div>
