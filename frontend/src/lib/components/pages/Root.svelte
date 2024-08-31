<script lang="ts">
	import type { Snippet } from 'svelte';
	import { setPagesState, type PagesClasses } from './context.svelte.ts';

	let { classes, children }: { classes?: PagesClasses; children: Snippet } = $props();

	setPagesState(classes);
</script>

<div>
	{@render children()}
</div>

<style lang="scss">
	div {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		gap: 20px;
	}
</style>
