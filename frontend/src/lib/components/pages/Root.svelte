<script lang="ts">
	import { type Snippet } from 'svelte';
	import { PagesState, type PagesClasses } from './context.svelte.ts';
	import { context } from './context.svelte.ts';

	let {
		currentPage = $bindable(0),
		classes,
		children
	}: { currentPage?: number; classes?: PagesClasses; children: Snippet } = $props();

	context.set(new PagesState(currentPage, classes));
</script>

<div class="flex h-full w-full flex-col gap-5">
	{@render children()}
</div>
