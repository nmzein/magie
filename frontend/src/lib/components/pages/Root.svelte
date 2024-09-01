<script lang="ts">
	import type { Snippet } from 'svelte';
	import { setPagesState, type PagesClasses } from './context.svelte.ts';

	let { classes, children }: { classes?: PagesClasses; children: Snippet } = $props();

	setPagesState(classes);
</script>

<div class="flex h-full w-full flex-col gap-5">
	{@render children()}
</div>
